use egui::{
    vec2, CentralPanel, Color32, Context, Frame, Layout, Margin, NumExt, Pos2,
    Rect, RichText, Rounding, Stroke, Ui,
};
use egui_extras::{RetainedImage, Size, StripBuilder};

use crate::{
    app_constants::{LIGHT_GREEN, SOFT_GREEN, WIN_HEIGHT, WIN_WIDTH},
    helper_functions::AppScale,
    hex_to_egui_rgb, scl, widgets,
};

pub struct WelcomeWindow {
    star: egui::Shape,
    show_hover: bool,
    time: Option<f64>,
}

impl WelcomeWindow {
    pub fn new() -> WelcomeWindow {
        WelcomeWindow {
            star: crate::shapes::draw_star(
                Pos2::ZERO,
                5,
                scl(792.),
                hex_to_egui_rgb(SOFT_GREEN),
            ),
            show_hover: true,
            time: Some(0.0),
        }
    }

    pub fn run_window(&mut self, ctx: &Context) {
        // Create custom frame for window
        let frame = Frame {
            fill: hex_to_egui_rgb(LIGHT_GREEN),
            inner_margin: Margin::symmetric(0., 0.),
            ..Default::default()
        };
        // Create Central panel
        let panel = CentralPanel::default().frame(frame);

        panel.show(ctx, |ui: &mut Ui| {
            // We need to allocate this before StripBuilder
            // for independent positioning.
            let mut child_ui = ui.child_ui(
                ui.max_rect(),
                Layout::left_to_right(egui::Align::Min),
            );
            // On original layer
            StripBuilder::new(ui)
                .sizes(Size::remainder(), 2)
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.with_layout(
                            Layout::centered_and_justified(
                                egui::Direction::TopDown,
                            ),
                            |ui| {
                                let button =
                                    ui.add(widgets::start_block(&self.star));
                                if button.clicked() {
                                    println!("Button is clicked!");
                                }
                                button.on_hover_text("Start training!");
                            },
                        );
                    });

                    strip.cell(|ui| {
                        let top_rect = guitar_rect(ui, -93., -286.);
                        let mid_rect = guitar_rect(ui, 256.5, 102.5);
                        let end_rect = guitar_rect(ui, -205.5, 330.);

                        let widget = widgets::guitar_block(
                            ("A", 4, 9, 7, 5),
                            (2, 2),
                            "Nailed it!",
                        );
                        let widget2 = widgets::guitar_block(
                            ("D", 2, 7, 5, 3),
                            (4, 4),
                            "Here you go!",
                        );
                        let widget3 = widgets::guitar_block(
                            ("Es", 5, 7, 5, 3),
                            (1, 1),
                            "Hover me",
                        );
                        if self.show_hover {
                            self.show_hover(end_rect, ctx);
                        }
                        widgets::connect(
                            ui,
                            top_rect.translate(vec2(0., 70.).scl()),
                            mid_rect,
                            egui::Direction::LeftToRight,
                        );
                        widgets::connect(
                            ui,
                            mid_rect.translate(vec2(0., -70.).scl()),
                            end_rect.translate(vec2(50., 50.).scl()),
                            egui::Direction::RightToLeft,
                        );
                        ui.put(mid_rect, widget2);
                        ui.put(top_rect, widget);
                        ui.put(end_rect, widget3);
                    });
                });
            // On other layer
            let settings_button = child_ui.add(
                egui::Button::new(RichText::new("Settings").size(scl(40.)))
                    .min_size(vec2(200., 50.).scl()),
            );
            if settings_button.clicked() {
                println!("Clicked!");
            }
        });
    }

    fn show_hover(&mut self, end_rect: Rect, ctx: &Context) {
        let hover_frame = Frame {
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::LIGHT_YELLOW,
            stroke: Stroke::new(scl(2.2), Color32::BLACK),
            rounding: Rounding::same(scl(10.)),
            inner_margin: Margin::same(5.),
            ..Default::default()
        };
        let time = self.time.unwrap();
        let opacity = egui::emath::remap(time, 0.0..=5.0, 0.0..=1.0) as f32;
        let window = egui::Window::new("hover")
            .title_bar(false)
            .resizable(false)
            .fixed_pos(end_rect.center() + vec2(-40., -4.))
            .frame(hover_frame.multiply_with_opacity(1. - opacity));
        window.show(ctx, |ui| {
            let time = self.time.unwrap();
            let opacity =
                egui::emath::remap(time, 0.0..=8.0, 0.0..=255.0) as u16;
            ui.label(RichText::new("Hover me!").color(
                Color32::from_rgba_premultiplied(
                    118,
                    118,
                    126,
                    (255 as u16 - opacity) as u8,
                ),
            ));
            let time = self.time.as_mut().unwrap();
            (*time) += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
            ctx.request_repaint();
            if (*time) >= 5. {
                self.show_hover = false;
                self.time = None;
            }
        });
    }
}

// ----- Helper functions --------------------------------------------------- //

/// This function applies scaling internally
fn guitar_rect(ui: &mut Ui, x: f32, y: f32) -> egui::Rect {
    let x = ui.max_rect().size().x * scl(x) / (WIN_WIDTH / 2.);
    let y = ui.max_rect().size().y * scl(y) / WIN_HEIGHT;
    ui.max_rect().translate(vec2(x, y))
}
