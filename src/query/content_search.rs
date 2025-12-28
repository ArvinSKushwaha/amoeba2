use crate::query::response::QueryResponse;
use crate::query::SearchEngine;
use egui::Ui;
use flume::Sender;
use futures::{AsyncBufReadExt, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Rga;

impl Rga {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum RgaJson {
    Match {
        data: RgaMatchData,
    },
    #[serde(other)] 
    Irrelevant
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RgaMatchData {
    path: Text,
    lines: Text,
    line_number: Option<u64>,
    absolute_offset: Option<u64>,
    submatches: Vec<Match>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Match {
    #[serde(rename = "match")]
    mat: Text,
    start: u64,
    end: u64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Text {
    text: String,
}


#[async_trait::async_trait]
impl SearchEngine for Rga {
    fn prefix(&self) -> &'static str {
        "@rg"
    }

    fn icon(&self) -> Box<dyn Fn(&mut Ui) -> egui::Response + Send> {
        Box::new(|ui| ui.monospace("󰈞"))
    }

    async fn search(&self, query: &str, channel: Sender<QueryResponse>) -> anyhow::Result<()> {
        let dir = std::env::current_dir().unwrap_or_default();
        log::info!("FileSystem Query: {}, cwd: {:?}", query, dir);

        let mut child = async_process::Command::new("rga")
            .arg("--json")
            .arg(query)
            .stdout(async_process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let mut lines = futures::io::BufReader::new(child.stdout.take().unwrap()).lines().enumerate();

        while let Some((i, line)) = lines.next().await {
            let line = line?;
            let parse: RgaJson = serde_json::from_str(&line)?;

            let rga_match = match parse {
                RgaJson::Irrelevant => continue,
                RgaJson::Match { data } => data,
            };
            let rga_match_clone = rga_match.clone();
            
            let icon = self.icon();

            let send_res = channel
                .send_async(QueryResponse::new(
                    {
                        Box::new(move |ui: &mut egui::Ui| {
                            icon(ui);

                            ui.add(
                                egui::Label::new(egui::RichText::new(format!("./{path}:", path = rga_match.path.text)).monospace().italics())
                                    .wrap_mode(egui::TextWrapMode::Wrap),
                            );

                            ui.add(
                                egui::Label::new(egui::RichText::new(format!("{match}", r#match = rga_match.lines.text.trim())).monospace())
                                    .wrap_mode(egui::TextWrapMode::Wrap),
                            )
                        })
                    },
                    {
                        let dir = dir.clone().to_string_lossy().to_string();
                        Box::new(move |ui: &mut egui::Ui| {
                            ui.ctx().send_cmd(egui::OutputCommand::CopyText(format!("{dir}/{path}", path = rga_match_clone.path.text)));
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
