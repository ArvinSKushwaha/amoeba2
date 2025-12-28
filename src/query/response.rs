use crate::theme::Theme;
use egui::{TextStyle, Ui};
use std::time::Duration;
use uuid::Uuid;

pub struct QueryResponse {
    pub duration: Option<Duration>,
    pub display: Box<dyn Fn(&mut Ui) -> egui::Response + Send>,
    pub action: Box<dyn Fn(&mut Ui) -> () + Send>,
    pub priority: i64,
    extra_state: Option<Vec<u8>>,
    uuid: Uuid,
}

impl std::fmt::Debug for QueryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryResponse")
            .field("duration", &self.duration)
            .field("display", &"..")
            .field("priority", &self.priority)
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl QueryResponse {
    pub fn new(
        widget: Box<impl Fn(&mut Ui) -> egui::Response + Send + 'static>,
        action: Box<impl Fn(&mut Ui) -> () + Send + 'static>,
        priority: i64,
    ) -> Self {
        Self {
            duration: None,
            display: widget,
            action,
            priority,
            extra_state: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn with_extra_state(mut self, state: Vec<u8>) -> Self {
        self.extra_state = Some(state);

        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);

        self
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_extra_state(&self) -> Option<&Vec<u8>> {
        self.extra_state.as_ref()
    }

    pub fn get_extra_state_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.extra_state.as_mut()
    }
}

fn format_duration(duration: Duration) -> String {
    match duration.as_millis() {
        0..5000 => format!("({i} ms)", i = duration.as_millis()),
        5000.. => format!("({i} s)", i = duration.as_secs()),
    }
}

impl QueryResponse {
    pub(crate) fn ui(
        &self,
        ui: &mut Ui,
        panel: egui::Frame,
        theme: &Theme,
        width: f32,
        active: bool,
    ) {
        let response = panel
            .show(ui, |ui| {
                ui.set_width(width - (theme.margin.left + theme.margin.right) as f32);
                ui.horizontal(|ui| {
                    
                    let data = if let Some(duration) = self.duration {
                        let small_style = TextStyle::Small.resolve(&ui.style());
                        let formatted_duration = format_duration(duration);
                        let text = egui::RichText::new(&formatted_duration).small();
                        let text_size = ui.fonts_mut(|f| f.layout_no_wrap(
                            formatted_duration,
                            small_style,
                            theme.noninteractive_fg_stroke.color().to(),
                        )).size();

                        Some((text, text_size))
                    } else {
                        None
                    };

                    ui.horizontal_wrapped(|ui| {
                        if let Some((_, size)) = data {
                            ui.set_max_width(ui.available_width() - size.x - theme.margin.x() as f32);
                        }
                        (self.display)(ui)
                    });

                    if let Some((text, size)) = data {
                        ui.add_space(ui.available_width() - size.x);
                        ui.label(text);
                    }
                });
            })
            .response;

        if active && ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
            (self.action)(ui)
        }

        if active && !ui.is_rect_visible(response.rect) {
            response.scroll_to_me(None)
        }
    }
}
