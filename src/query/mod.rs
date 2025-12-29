mod content_search;
mod file_search;
mod mock_engine;
mod wikipedia;

use crate::query::content_search::Rga;
use crate::query::file_search::Fzf;
use crate::query::mock_engine::MockEngine;
use crate::query::wikipedia::WikipediaEngine;
use crate::response::QueryResponse;
use egui::Ui;
use flume::Receiver;
use futures::executor::ThreadPool;
use futures::future::{RemoteHandle, join_all};
use futures::task::SpawnExt;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryConfig {
    pool_size: usize,
}

impl Default for QueryConfig {
    fn default() -> Self {
        QueryConfig { pool_size: 0 }
    }
}

pub struct EngineCollection(HashMap<&'static str, Vec<Box<dyn SearchEngine + Sync + Send>>>);

lazy_static::lazy_static! {
    pub static ref ENGINES: RwLock<EngineCollection> = RwLock::new({
        let engines: Vec<Box<dyn SearchEngine + Sync + Send>> = vec![
            Box::new(MockEngine),
            Box::new(WikipediaEngine::new()),
            Box::new(Fzf::new()),
            Box::new(Rga::new()),
        ];

        let mut map = HashMap::new();
        for engine in engines.into_iter() {
            map.entry(engine.prefix()).or_insert(Vec::new()).push(engine);
        }

        EngineCollection(map)
    });
}

pub struct QueryState {
    pub _handle: RemoteHandle<()>,
    pub rcv: Receiver<QueryResponse>,
}

pub struct QueryEngine {
    thread_pool: ThreadPool,
    query_state: RwLock<Option<QueryState>>,
}

impl std::fmt::Debug for QueryEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryEngine")
            .field("responses", &"..")
            .field("engines", &"..")
            .finish()
    }
}

impl QueryEngine {
    pub fn new(config: &QueryConfig) -> Self {
        QueryEngine {
            thread_pool: ThreadPool::builder()
                .pool_size(if config.pool_size == 0 {
                    num_cpus::get()
                } else {
                    config.pool_size
                })
                .create()
                .expect("Failed to create thread pool"),
            query_state: RwLock::new(None),
        }
    }

    pub fn icon(&self, filter: &Option<String>, ui: &mut Ui) {
        if let Some(filter) = filter {
            if ENGINES.read().0.contains_key(filter.as_str())
                && let Some(engine) = ENGINES.read().0[filter.as_str()].first()
            {
                engine.icon()(ui);
            } else {
                ui.monospace("");
            }
        } else {
            ui.monospace("󰍉");
        }
    }

    pub async fn query_async(
        query: String,
        filter: Option<String>,
        snd: flume::Sender<QueryResponse>,
    ) {
        let _ = join_all(
            ENGINES
                .deref()
                .read()
                .0
                .iter()
                .filter_map(|(&prefix, engines)| match &filter {
                    None => Some(engines),
                    Some(p) if p == prefix => Some(engines),
                    _ => None,
                })
                .flatten()
                .map(|engine| async {
                    if let Err(e) = engine.search(&query, snd.clone()).await {
                        log::error!("Query Error: {e}");
                    }
                }),
        )
        .await;
    }

    pub fn query(&mut self, query: &str, filter: &Option<String>) {
        log::info!("Query: {}", query);
        let (snd, rcv) = flume::bounded(1024);

        if let Ok(handle) = self
            .thread_pool
            .spawn_with_handle(QueryEngine::query_async(
                query.to_string(),
                filter.clone(),
                snd,
            ))
            .inspect_err(|e| log::error!("{e}"))
        {
            self.query_state.write().replace(QueryState {
                _handle: handle,
                rcv,
            });
        }
    }

    pub fn clear_query(&mut self) {
        drop(self.query_state.write().take());
    }

    pub fn match_receiver(&self, rcv: &Receiver<QueryResponse>) -> bool {
        self.query_state
            .read()
            .as_ref()
            .map(|QueryState { rcv: base_rcv, .. }| base_rcv.same_channel(rcv))
            .unwrap_or(false)
    }

    pub fn responses(&self) -> Option<Receiver<QueryResponse>> {
        if let Some(QueryState { rcv, .. }) = &*self.query_state.read() {
            Some(rcv.clone())
        } else {
            None
        }
    }
}

#[async_trait::async_trait]
pub trait SearchEngine {
    fn name(&self) -> &'static str;
    fn prefix(&self) -> &'static str;
    fn icon(&self) -> Box<dyn Fn(&mut Ui) -> egui::Response + Send>;

    async fn search(
        &self,
        query: &str,
        channel: flume::Sender<QueryResponse>,
    ) -> anyhow::Result<()>;
}
