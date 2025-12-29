use crate::query::SearchEngine;
use crate::response::QueryResponse;
use egui::Ui;
use flume::Sender;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use strfmt::strfmt;
use surf::{Client, Config, Url};

lazy_static! {
    pub static ref WIKIMEDIA_CLIENT_ID: Option<String> = std::env::var("WIKIMEDIA_CLIENT_ID").ok();
    pub static ref WIKIMEDIA_CLIENT_SECRET: Option<String> =
        std::env::var("WIKIMEDIA_CLIENT_SECRET").ok();
    pub static ref WIKIMEDIA_ACCESS_TOKEN: Option<String> =
        std::env::var("WIKIMEDIA_ACCESS_TOKEN").ok();
}

pub const WIKIMEDIA_URL: &str = "https://api.wikimedia.org";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResults {
    pages: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    id: u64,
    key: String,
    title: String,
    excerpt: String,
    matched_title: Option<String>,
    description: Option<String>,
    thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumbnail {
    mimetype: String,
    size: Option<u64>,
    width: Option<u64>,
    height: Option<u64>,
    duration: Option<u64>,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchContent {
    project: String,
    language: String,
    #[serde(rename = "q")]
    query: String,
    limit: Option<u8>,
}

impl SearchContent {
    pub const ENDPOINT: &'static str = "/core/v1/{project}/{language}/search/page";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchTitle {
    project: String,
    language: String,
    #[serde(rename = "q")]
    query: String,
    limit: Option<u8>,
}

impl SearchTitle {
    pub const ENDPOINT: &'static str = "/core/v1/{project}/{language}/search/title";
}

#[derive(Debug)]
pub struct WikipediaEngine {
    client: Option<Client>,
}

impl WikipediaEngine {
    pub fn new() -> Self {
        let client = (|| {
            let mut config = Config::new()
                .set_base_url(
                    Url::parse(WIKIMEDIA_URL)
                        .inspect_err(|e| log::error!("{e}"))
                        .ok()?,
                )
                .set_timeout(Some(Duration::from_secs(5)));

            if let Some(access_token) = &*WIKIMEDIA_ACCESS_TOKEN {
                config = config
                    .add_header("User-Agent", "Amoeba (me@arvinsk.org)")
                    .inspect_err(|e| log::error!("{e}"))
                    .ok()?
                    .add_header("Authorization", format!("Bearer {access_token}"))
                    .inspect_err(|e| log::error!("{e}"))
                    .ok()?;
            }

            config.try_into().ok()
        })();

        Self { client }
    }
}

#[async_trait::async_trait]
impl SearchEngine for WikipediaEngine {
    fn name(&self) -> &'static str {
        "wikipedia"
    }

    fn prefix(&self) -> &'static str {
        "@wi"
    }

    fn icon(&self) -> Box<dyn Fn(&mut Ui) -> egui::Response + Send> {
        Box::new(|ui| ui.monospace("󰖬"))
    }

    async fn search(&self, query: &str, channel: Sender<QueryResponse>) -> anyhow::Result<()> {
        log::info!("WikipediaEngine Query: {}", query);

        futures_timer::Delay::new(Duration::from_millis(720)).await;

        log::info!("Making request");

        let search = SearchTitle {
            project: "wikipedia".to_string(),
            language: "en".to_string(),
            query: query.to_string(),
            limit: None,
        };

        if let Some(client) = &self.client {
            let request = client.get(strfmt!(SearchTitle::ENDPOINT, project => search.project.clone(), language => search.language.clone())?)
                .query(&search)
                .map_err(|err| anyhow::anyhow!(err))?
                .build();

            let mut response = client
                .send(request)
                .await
                .map_err(|err| anyhow::anyhow!(err))?;

            log::trace!("Response: {response:?}");

            let res: SearchResults = response
                .body_json()
                .await
                .map_err(|err| anyhow::anyhow!(err))?;

            for (i, res) in res.pages.into_iter().enumerate() {
                let icon = self.icon();
                let send_res = channel
                    .send_async(QueryResponse::new(
                        {
                            let res = res.clone();
                            Box::new(move |ui: &mut egui::Ui| {
                                icon(ui);

                                ui.add(
                                    egui::Label::new(egui::RichText::new(
                                        format!(
                                            "({title}) {desc}",
                                            title = res.title,
                                            desc = if let Some(desc) = res.description.as_ref() {
                                                desc
                                            } else {
                                                ""
                                            }
                                        )
                                        .trim(),
                                    ))
                                    .wrap_mode(egui::TextWrapMode::Wrap),
                                )
                            })
                        },
                        {
                            let search = search.clone();
                            Box::new(move |ui: &mut egui::Ui| {
                                ui.ctx().send_cmd(egui::OutputCommand::OpenUrl(
                                    egui::OpenUrl::new_tab(format!(
                                        "https://{language}.wikipedia.org/wiki/{title}",
                                        language = &search.language,
                                        title = &res.key
                                    )),
                                ));
                            })
                        },
                        5 - (i as i64),
                    ))
                    .await;

                if let Err(err) = send_res {
                    return Err(anyhow::anyhow!("Err: {}", err));
                }
            }
        }

        Ok(())
    }
}
