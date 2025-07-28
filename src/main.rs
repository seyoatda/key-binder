pub mod key;

mod config;
mod gui;

use eframe::NativeOptions;
use egui::Pos2;
use key::bind_key_sets;
use key::VirtualKey;
use key::VirtualKeySet;
use std::path::Path;
use std::thread;

use crate::config::read_config;
use crate::config::Config;
use crate::gui::app::GuiApp;
use crate::key::listen_event;

fn main() {
    init_key_mappings();

    thread::spawn(|| {
        listen_event();
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_always_on_top()
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_decorations(false)
            // .with_position(Pos2::new(0.0, 800.0))
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Key Binder",
        native_options,
        Box::new(|cc| {
            // 设置完全透明的背景
            cc.egui_ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::TRANSPARENT,
                panel_fill: egui::Color32::TRANSPARENT,
                ..Default::default()
            });
            Ok(Box::new(GuiApp::default()))
        }),
    )
    .unwrap();
}

fn init_key_mappings() {
    let config = read_config(Path::new("./resources/config.toml"));
    for x in config.key_mappings {
        bind_key_sets(x.origin.keys.as_slice(), x.mapping.keys.as_slice())
    }
}
