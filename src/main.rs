// neck_rust binary
// main.rs

use eframe::{CreationContext, NativeOptions, Renderer};
use egui::vec2;
use neck_rust::{
    app_constants::{MIN_HEIGHT, MIN_WIDTH, WIN_HEIGHT, WIN_WIDTH},
    NeckRust,
};

fn main() {
    let options = NativeOptions {
        renderer: Renderer::Wgpu,
        initial_window_size: Some(vec2(WIN_WIDTH, WIN_HEIGHT)),
        min_window_size: Some(vec2(MIN_WIDTH, MIN_HEIGHT)),
        ..Default::default()
    };

    eframe::run_native(
        "neck_rust",
        options,
        Box::new(|cc: &CreationContext| Box::new(NeckRust::new(cc))),
    )
    .expect("error run_native");
}
