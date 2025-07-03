#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, NativeOptions};
use egui::ViewportBuilder;

mod ui;
mod network_info;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Network Information",
        options,
        Box::new(|_cc| Box::new(NetworkInfoApp::default())),
    )
    .expect("Failed to run eframe");
}

pub struct NetworkInfoApp {
    network_data: network_info::NetworkData,
}

impl Default for NetworkInfoApp {
    fn default() -> Self {
        Self {
            network_data: network_info::NetworkData::new(),
        }
    }
}

impl eframe::App for NetworkInfoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.network_data.refresh();
        ui::render(ctx, &self.network_data);
    }
}