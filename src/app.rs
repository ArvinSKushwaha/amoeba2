use crate::config::AmoebaConfig;
use crate::query::response::QueryResponse;
use crate::query::QueryEngine;
use crate::theme::{CornerRadius, Margin};
use eframe::epaint::text::{FontInsert, InsertFontFamily};
use eframe::{App, CreationContext, Frame};
use egui::epaint::text::FontPriority;
use egui::{Context, FontFamily, ViewportCommand, Visuals};
use egui::{TextEdit, TextStyle};
use flume::Receiver;

#[derive(Debug)]
pub struct AmoebaApp {
    width: f32,
    query_engine: QueryEngine,
    receiver: Option<Receiver<QueryResponse>>,
    responses: Vec<QueryResponse>,
    filter: Option<String>,
    active: Option<uuid::Uuid>,
    config: AmoebaConfig,
    query_bar: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AmoebaAppCreationError {}

impl AmoebaApp {
    pub fn new(cc: &CreationContext, config: AmoebaConfig) -> Result<Self, AmoebaAppCreationError> {
        cc.egui_ctx
            .all_styles_mut(|style| config.theme.update(style));
        cc.egui_ctx.add_font(FontInsert::new(
            "FiraCode Nerd Font Mono",
            egui::FontData::from_static(include_bytes!(
                "../FiraCode/FiraCodeNerdFontMono-Regular.ttf"
            )),
            vec![InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Highest,
            }],
        ));

        Ok(AmoebaApp {
            width: 0.,
            query_engine: QueryEngine::new(&config.query_config),
            filter: None,
            query_bar: String::new(),
            receiver: None,
            active: None,
            responses: Vec::with_capacity(1024),
            config,
        })
    }

    fn request_query(&mut self) {
        self.query_engine.query(&self.query_bar, &self.filter);
    }

    fn clear_query(&mut self) {
        self.query_engine.clear_query();
    }
}

impl App for AmoebaApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(rcv) = &self.receiver
            && self.query_engine.match_receiver(rcv)
        {
            self.responses.extend(rcv.drain());
            self.responses.sort_by_key(|i| -i.priority);
        } else {
            self.active = None;
            self.receiver = self.query_engine.responses();
            self.responses.clear();
        }

        let have_responses = !self.responses.is_empty();

        let mut active_idx = None;

        let query_panel_frame = egui::Frame::NONE
            .fill(ctx.style().visuals.window_fill())
            .stroke(egui::Stroke::NONE)
            .shadow(egui::Shadow::NONE)
            .inner_margin(self.config.theme.margin)
            .outer_margin(Margin::symmetric(0, -2))
            .corner_radius(if have_responses {
                self.config.theme.query_corner_radius_with_results
            } else {
                self.config.theme.query_corner_radius
            });

        egui::TopBottomPanel::top("Amoeba")
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                query_panel_frame.show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(2.0);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(2.0);

                                ui.horizontal(|ui| {
                                    self.query_engine.icon(&self.filter, ui);
                                });
                            });

                            let response = ui.add_sized(
                                [
                                    ui.available_width(),
                                    ui.text_style_height(&TextStyle::Body) + 4.0,
                                ],
                                TextEdit::singleline(&mut self.query_bar)
                                    .font(TextStyle::Monospace)
                                    .return_key(None)
                                    .desired_width(f32::INFINITY)
                                    .lock_focus(true)
                                    .frame(false)
                                    .id_source("query_edit"),
                            );

                            response.request_focus();

                            if response.has_focus() {
                                if let Some(mut state) = TextEdit::load_state(ctx, response.id)
                                    && let Some(range) = state.cursor.char_range()
                                    && let Some(pos) = range.single()
                                {
                                    if pos.index == 0
                                        && ctx.input(|i| {
                                            i.key_pressed(egui::Key::Backspace)
                                                || i.key_pressed(egui::Key::ArrowLeft)
                                        })
                                    {
                                        let filter = self.filter.take();
                                        if let Some(filter) = filter {
                                            state.cursor.set_char_range(Some(
                                                egui::text::CCursorRange::one(
                                                    egui::text::CCursor::new(filter.len()),
                                                ),
                                            ));
                                            self.query_bar = filter + " " + &self.query_bar;
                                            state.store(ctx, response.id);
                                            self.request_query();
                                        }
                                        log::info!(
                                            "Filter: {:?}, Query Bar: {:?}",
                                            self.filter,
                                            self.query_bar
                                        );
                                    } else if self.query_bar.starts_with('@')
                                        && let Some(idx) = self.query_bar.find(' ')
                                        && pos.index > idx
                                    {
                                        state.cursor.set_char_range(Some(
                                            egui::text::CCursorRange::one(pos - idx - 1),
                                        ));
                                        let mut tmp = self.query_bar.split_off(idx + 1);
                                        std::mem::swap(&mut self.query_bar, &mut tmp);
                                        tmp.pop();
                                        self.filter.replace(tmp);
                                        state.store(ctx, response.id);

                                        log::info!(
                                            "Filter: {:?}, Query Bar: {:?}",
                                            self.filter,
                                            self.query_bar
                                        );

                                        self.request_query();
                                    }
                                }
                            }

                            if response.changed() || response.lost_focus() {
                                if !self.query_bar.trim().is_empty() {
                                    self.request_query();
                                } else {
                                    self.clear_query();
                                }
                            }
                        });
                    });
                });

                egui::ScrollArea::vertical()
                    .min_scrolled_height(self.config.theme.max_height)
                    .show(ui, |ui| {
                        for (i, resp) in self.responses.iter().enumerate() {
                            let is_active = if let Some(active) = self.active
                                && active == resp.get_uuid()
                            {
                                active_idx = Some(i);
                                true
                            } else {
                                false
                            };

                            let panel_frame = egui::Frame::NONE
                                .fill(
                                    if is_active {
                                        self.config.theme.active_bg_fill
                                    } else {
                                        self.config.theme.window_fill
                                    }
                                    .to(),
                                )
                                .stroke(egui::Stroke::NONE)
                                .shadow(egui::Shadow::NONE)
                                .inner_margin(self.config.theme.margin)
                                .outer_margin(Margin::symmetric(0, -2))
                                .corner_radius(
                                    if i < self.responses.len() - 1 {
                                        CornerRadius::NONE
                                    } else {
                                        self.config.theme.query_corner_radius_with_results.flip_y()
                                    }
                                    .to(),
                                );

                            resp.ui(ui, panel_frame, &self.config.theme, self.width, is_active);
                        }
                    });
            });

        if ctx.input(|i| i.keys_down.contains(&egui::Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        if have_responses {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if let Some(active) = active_idx
                    && active < self.responses.len() - 1
                {
                    self.active.replace(self.responses[active + 1].get_uuid());
                } else {
                    self.active.replace(self.responses[0].get_uuid());
                }
            } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(active) = active_idx
                    && active > 0
                {
                    self.active.replace(self.responses[active - 1].get_uuid());
                } else {
                    self.active
                        .replace(self.responses[self.responses.len() - 1].get_uuid());
                }
            }
        }

        self.width = ctx.viewport_rect().width();
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(
            (self.width, ctx.used_size().y).into(),
        ));
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.; 4]
    }
}
