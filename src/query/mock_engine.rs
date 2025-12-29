use crate::query::SearchEngine;
use crate::response::QueryResponse;
use egui::Ui;
use flume::Sender;
use std::time::Instant;

#[derive(Debug, Default)]
pub struct MockEngine;

#[async_trait::async_trait]
impl SearchEngine for MockEngine {
    fn name(&self) -> &'static str {
        "mock_engine"
    }

    fn prefix(&self) -> &'static str {
        "@mk"
    }

    fn icon(&self) -> Box<dyn Fn(&mut Ui) -> egui::Response + Send> {
        Box::new(|ui| ui.monospace("󰤑"))
    }

    async fn search(&self, query: &str, channel: Sender<QueryResponse>) -> anyhow::Result<()> {
        let start = Instant::now();
        log::info!("Received search request: {query}");

        let loc_query = query.to_string();
        let icon = self.icon();
        let res = channel
            .send_async(
                QueryResponse::new(
                    Box::new(move |ui: &mut egui::Ui| {
                        icon(ui);

                        ui.add(
                            egui::Label::new(egui::RichText::new(loc_query.to_string()))
                                .wrap_mode(egui::TextWrapMode::Wrap),
                        )
                    }),
                    Box::new(|_: &mut egui::Ui| {}),
                    0,
                )
                .with_duration(start.elapsed()),
            )
            .await;

        if let Err(err) = res {
            return Err(anyhow::anyhow!("Err: {}", err));
        }

        log::info!("Completed query: {query}");
        Ok(())
    }
}
