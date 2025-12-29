use crate::query::SearchEngine;
use crate::response::QueryResponse;
use egui::Ui;
use flume::Sender;
use futures::{AsyncBufReadExt, StreamExt};

#[derive(Debug)]
pub struct Fzf;

impl Fzf {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl SearchEngine for Fzf {
    fn name(&self) -> &'static str {
        "fzf"
    }

    fn prefix(&self) -> &'static str {
        "@fzf"
    }

    fn icon(&self) -> Box<dyn Fn(&mut Ui) -> egui::Response + Send> {
        Box::new(|ui| ui.monospace(""))
    }

    async fn search(&self, query: &str, channel: Sender<QueryResponse>) -> anyhow::Result<()> {
        let dir = std::env::current_dir().unwrap_or_default();
        log::info!("FileSystem Query: {}, cwd: {:?}", query, dir);

        let mut child = async_process::Command::new("fzf")
            .arg("-f")
            .arg(query)
            .stdout(async_process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let mut lines = futures::io::BufReader::new(child.stdout.take().unwrap())
            .lines()
            .enumerate();

        while let Some((i, path)) = lines.next().await {
            let path = path?;
            let path_clone = path.clone();

            let icon = self.icon();

            let send_res = channel
                .send_async(QueryResponse::new(
                    {
                        Box::new(move |ui: &mut egui::Ui| {
                            icon(ui);

                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!("./{path}"))
                                        .monospace()
                                        .italics(),
                                )
                                .wrap_mode(egui::TextWrapMode::Wrap),
                            )
                        })
                    },
                    {
                        let dir = dir.clone().to_string_lossy().to_string();
                        Box::new(move |ui: &mut egui::Ui| {
                            ui.ctx().send_cmd(egui::OutputCommand::CopyText(format!(
                                "{dir}/{path_clone}"
                            )));
                        })
                    },
                    5 - (i as i64),
                ))
                .await;

            if let Err(err) = send_res {
                return Err(anyhow::anyhow!("Err: {}", err));
            }
        }

        Ok(())
    }
}
