mod app;
mod config;
mod query;
mod response;
mod theme;

use crate::app::AmoebaApp;
use crate::config::AmoebaConfig;
use eframe::{NativeOptions, Renderer, egui, run_native};
use egui::{ViewportBuilder, WindowLevel, X11WindowType};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    #[cfg(debug_assertions)]
    env_logger::Builder::from_default_env()
        .filter_module("amoeba", log::LevelFilter::Trace)
        .init();

    let config: AmoebaConfig = confy::load("amoeba", Some("config"))?;

    let err = run_native(
        "Amoeba",
        NativeOptions {
            centered: true,
            renderer: Renderer::Wgpu,
            viewport: ViewportBuilder {
                active: Some(true),
                decorations: Some(false),
                mouse_passthrough: Some(false),
                titlebar_shown: Some(true),
                transparent: Some(true),
                window_level: Some(WindowLevel::AlwaysOnTop),
                window_type: Some(X11WindowType::Dock),
                app_id: Some("amoeba".to_string()),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(AmoebaApp::new(cc, config)?))),
    );

    if let Err(ref err) = err {
        anyhow::bail!("Initialization Error: {err}");
    }

    Ok(())
}
