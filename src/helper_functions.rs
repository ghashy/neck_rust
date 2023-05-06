// ----- Helper functions --------------------------------------------------- //

use egui::Color32;

use crate::app_constants::SCALE_FACTOR;

pub fn hex_to_rgb(color: u32) -> [f32; 4] {
    // Dividing for compression in 0.0..1.0 range
    let r = ((color >> 16) & 0xff) as f32 / 255.0;
    let g = ((color >> 8) & 0xff) as f32 / 255.0;
    let b = ((color) & 0xff) as f32 / 255.0;
    [r, g, b, 1.0]
}

pub fn hex_to_egui_rgb(color: u32) -> Color32 {
    // Dividing for compression in 0.0..1.0 range
    let r = ((color >> 16) & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = ((color) & 0xff) as u8;
    Color32::from_rgb(r, g, b)
}

/// Scale physical pixel's value to logic size.
pub fn scl(value: f32) -> f32 {
    value / SCALE_FACTOR
}

// ----- Scaling ------------------------------------------------------------ //

pub trait AppScale {
    fn scl(self) -> Self;
}

impl AppScale for f32 {
    fn scl(self) -> Self {
        self / SCALE_FACTOR
    }
}

impl AppScale for egui::emath::Vec2 {
    fn scl(self) -> Self {
        egui::Vec2 {
            x: self.x / SCALE_FACTOR,
            y: self.y / SCALE_FACTOR,
        }
    }
}

impl AppScale for egui::Pos2 {
    fn scl(self) -> Self {
        egui::pos2(self.x / SCALE_FACTOR, self.y / SCALE_FACTOR)
    }
}
