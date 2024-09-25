use anyhow::Result;
use eframe::{egui, App};

use crate::errors::to_anyhow;
use crate::widgets::main_panel::MainPanel;
use crate::widgets::view::View;

pub fn run_app(title: &str) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_position([448.0, 156.0 - 50.0]),
        ..Default::default()
    };

    eframe::run_native(title, options, Box::new(|cc| Ok(Box::new(BunyiApp::new(cc)))))
        .map_err(to_anyhow)
}

#[derive(Default)]
struct BunyiApp {
    main_panel: MainPanel,
}

impl BunyiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl App for BunyiApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_panel.show(ctx);
    }
}
