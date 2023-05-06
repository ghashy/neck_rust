use app_constants::{BLUE, BLUE_HOVER, BLUE_PRESSED, HOVER_GREEN, LIGHT_GREEN};
use eframe::{self, CreationContext};
use egui::{
    Color32, Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle,
    Visuals,
};
use helper_functions::{hex_to_egui_rgb, hex_to_rgb, scl};

// ----- Submodules --------------------------------------------------------- //

pub mod app_constants;
pub mod helper_functions;
pub mod paint_bezier;
pub mod plot_demo;
pub mod shapes;
pub mod strip;
pub mod switch_widget;
pub mod welcome_window;
pub mod widgets;

// ----- Body --------------------------------------------------------------- //

pub struct NeckRust {
    welcome_window: welcome_window::WelcomeWindow,
}

impl NeckRust {
    pub fn new(cc: &CreationContext) -> NeckRust {
        Self::setup_fonts(cc);
        Self::setup_visuals(cc);
        Self {
            welcome_window: welcome_window::WelcomeWindow::new(),
        }
    }

    fn setup_fonts(cc: &CreationContext) {
        let mut fonts = FontDefinitions::default();
        let font = include_bytes!(
            "../assets/fonts/M_A/MontserratAlternates-Regular.ttf"
        );
        let font_bold =
            include_bytes!("../assets/fonts/M_A/MontserratAlternates-Bold.ttf");
        fonts.font_data.insert(
            "Montserrat Alternates".to_string(),
            FontData::from_static(font),
        );
        fonts.font_data.insert(
            "Montserrat Alternates-bold".to_string(),
            FontData::from_static(font_bold),
        );
        fonts.families.insert(
            FontFamily::Proportional,
            vec!["Montserrat Alternates".to_string()],
        );
        fonts.families.insert(
            FontFamily::Name("Montserrat Alternates-bold".into()),
            vec!["Montserrat Alternates-bold".to_string()],
        );
        let mut style = (*cc.egui_ctx.style()).clone();
        let text_style = [
            (
                TextStyle::Heading,
                FontId::new(25., egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(25., egui::FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(18., egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(12., egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(20., egui::FontFamily::Proportional),
            ),
        ];
        style.text_styles = text_style.into();
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_fonts(fonts);
    }

    fn setup_visuals(cc: &CreationContext) {
        let widget_visuals = egui::style::WidgetVisuals {
            bg_fill: Color32::BLACK,
            weak_bg_fill: Color32::WHITE,
            bg_stroke: egui::Stroke::new(scl(2.2), Color32::RED),
            rounding: egui::Rounding::none(),
            fg_stroke: egui::Stroke::new(scl(2.2), Color32::GOLD),
            expansion: scl(2.2),
        };
        let visuals = Visuals {
            widgets: egui::style::Widgets {
                inactive: egui::style::WidgetVisuals {
                    bg_fill: hex_to_egui_rgb(HOVER_GREEN),
                    bg_stroke: egui::Stroke::new(scl(2.2), Color32::BLACK),
                    weak_bg_fill: hex_to_egui_rgb(BLUE),
                    ..widget_visuals
                },
                hovered: egui::style::WidgetVisuals {
                    bg_fill: Color32::BLACK,
                    weak_bg_fill: hex_to_egui_rgb(BLUE_HOVER),
                    bg_stroke: egui::Stroke::new(scl(2.2), Color32::WHITE),
                    ..widget_visuals
                },
                active: egui::style::WidgetVisuals {
                    weak_bg_fill: hex_to_egui_rgb(BLUE_PRESSED),
                    bg_stroke: egui::Stroke::new(scl(2.2), Color32::WHITE),
                    ..widget_visuals
                },
                ..Default::default()
            },
            dark_mode: false,
            window_fill: Color32::LIGHT_YELLOW,
            popup_shadow: egui::epaint::Shadow::small_light(),
            ..Default::default()
        };
        cc.egui_ctx.set_visuals(visuals);
    }
}

impl eframe::App for NeckRust {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.welcome_window.run_window(ctx);
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        hex_to_rgb(LIGHT_GREEN)
    }
}
