//! `widgets`: widgets for neck_rust.
//!

use crate::app_constants::{DARK_GREEN, MID_GREEN, SKY_GREEN, SWAMP_GREEN};
use crate::helper_functions::AppScale;
use crate::{hex_to_egui_rgb, scl};
use egui::epaint::Rect;
use egui::{
    pos2, vec2, Align2, Direction, FontId, Layout, Pos2, Response, Shape,
};
use egui::{Color32, Rounding, Sense, Stroke};
use egui_extras::RetainedImage;

fn draw_start_block(ui: &mut egui::Ui, star: &Shape) -> Response {
    // Parent (star) object's properties
    let star_size = vec2(792., 792.).scl();
    let (rect, response) = ui.allocate_exact_size(star_size, Sense::click());

    let response = if ui.is_rect_visible(rect) {
        // Draw a star
        let mut star = star.clone();
        star.translate(vec2(rect.center().x - scl(30.), rect.center().y));
        ui.painter().add(star);

        // Draw a rect
        let rect = rect.shrink(scl(154.)).expand2(vec2(39.6, 2.2).scl());
        ui.painter().rect(
            rect,
            Rounding::none(),
            Color32::TRANSPARENT,
            Stroke::new(scl(8.14), hex_to_egui_rgb(DARK_GREEN)),
        );

        // Draw top box
        let rect = rect
            .shrink2(vec2(17.6, 193.6).scl())
            .translate(vec2(0.0, -169.4).scl());
        ui.painter().rect(
            rect,
            scl(19.8),
            hex_to_egui_rgb(MID_GREEN),
            Stroke::NONE,
        );

        // Draw top label
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "Learn the neck",
            egui::FontId::proportional(scl(66.)),
            hex_to_egui_rgb(DARK_GREEN),
        );

        // Draw about box
        let rect = rect
            .expand2(vec2(-99., 105.6).scl())
            .translate(vec2(99., 231.).scl());
        ui.painter().rect(
            rect,
            scl(19.8),
            hex_to_egui_rgb(MID_GREEN),
            Stroke::NONE,
        );

        // Draw about label
        let about_rect = ui.painter_at(rect).text(
            rect.center_top() + vec2(0., 8.8).scl(),
            Align2::CENTER_TOP,
            "About:",
            egui::FontId::proportional(scl(44.)),
            hex_to_egui_rgb(DARK_GREEN),
        );

        // Draw about text
        let text = "This is an app made for guitarists to learn every \
                    note on every string. The letter is the note and \
                    the number is the string. Are you ready?"
            .to_string();
        let galley = ui.painter().layout(
            text,
            egui::FontId::proportional(scl(27.8)),
            hex_to_egui_rgb(DARK_GREEN),
            rect.width() - scl(22.),
        );
        ui.painter()
            .galley(about_rect.left_bottom() - vec2(74.8, 0.).scl(), galley);

        // Draw a button
        let rect = rect
            .shrink2(vec2(77., 0.).scl())
            .translate(vec2(-272.8, 0.).scl());
        let layout = Layout::left_to_right(egui::Align::Min);
        let response = ui
            .child_ui(rect, layout)
            .allocate_response(rect.size(), Sense::click());
        let visuals = ui.style().interact(&response);
        ui.painter().rect(
            rect.expand(visuals.expansion),
            scl(19.8),
            visuals.weak_bg_fill,
            visuals.bg_stroke,
        );

        // Draw button's text, and create bold text FontId
        let bold_font_id = FontId {
            size: scl(55.),
            family: egui::FontFamily::Name("Montserrat Alternates-bold".into()),
        };
        let galley = ui.painter().layout(
            "let's go!".to_string(),
            bold_font_id,
            Color32::WHITE,
            rect.width() - scl(22.),
        );
        ui.painter()
            .galley(rect.left_top() + vec2(33., 79.2).scl(), galley);
        response
    } else {
        return response;
    };
    response
}

fn draw_guitar_block(
    ui: &mut egui::Ui,
    data: (&'static str, u8, u8, u8, u8),
    hover_xy: (u8, u8),
    hover_string: &'static str,
) -> Response {
    let (rect, _) =
        ui.allocate_exact_size(vec2(359., 308.).scl(), Sense::hover());

    // Draw main rect
    ui.painter().rect(
        rect,
        scl(19.8),
        hex_to_egui_rgb(SKY_GREEN),
        Stroke::NONE,
    );

    // Draw left-top circle
    ui.painter().circle(
        rect.min + vec2(96., 53.).scl(),
        scl(26.),
        hex_to_egui_rgb(MID_GREEN),
        Stroke::NONE,
    );

    // Draw left-top string
    ui.painter().text(
        rect.min + vec2(96., 53.).scl(),
        Align2::CENTER_CENTER,
        data.0,
        FontId::proportional(scl(35.)),
        hex_to_egui_rgb(DARK_GREEN),
    );

    // Draw small line
    ui.painter().text(
        rect.min + vec2(172., 55.).scl(),
        Align2::CENTER_CENTER,
        "-",
        FontId::proportional(scl(39.)),
        hex_to_egui_rgb(DARK_GREEN),
    );

    // Draw right-top circle
    ui.painter().circle(
        rect.min + vec2(248., 53.).scl(),
        scl(26.),
        hex_to_egui_rgb(MID_GREEN),
        Stroke::NONE,
    );

    // Draw right-top number
    ui.painter().text(
        rect.min + vec2(248., 53.).scl(),
        Align2::CENTER_CENTER,
        data.1.to_string(),
        FontId::proportional(scl(35.)),
        hex_to_egui_rgb(DARK_GREEN),
    );

    // Draw table-rect
    let table_org = rect.min + vec2(23., 92.).scl();
    let table_width = scl(300.);
    let table_height = scl(200.);
    ui.painter().rect(
        Rect::from_min_size(table_org, vec2(table_width, table_height)),
        Rounding::none(),
        Color32::TRANSPARENT,
        Stroke::new(scl(2.), hex_to_egui_rgb(DARK_GREEN)),
    );

    // Draw vertical lines
    for i in 1..6 {
        ui.painter().vline(
            table_org.x + scl(50.) * i as f32,
            table_org.y..=table_org.y + vec2(0., table_height).y,
            Stroke::new(scl(2.), hex_to_egui_rgb(SWAMP_GREEN)),
        );
    }

    // Draw horizontal lines
    for i in 1..5 {
        ui.painter().hline(
            table_org.x..=table_org.x + vec2(table_width, 0.).x,
            table_org.y + scl(40. * i as f32),
            Stroke::new(scl(2.), hex_to_egui_rgb(DARK_GREEN)),
        );
    }

    // Draw three small circles
    for i in 0..3 {
        ui.painter().circle(
            table_org + vec2(26., 99.).scl() + vec2(100. * i as f32, 0.).scl(),
            scl(14.),
            hex_to_egui_rgb(SWAMP_GREEN),
            Stroke::NONE,
        );
        let number = match i {
            0 => data.2,
            1 => data.3,
            2 => data.4,
            _ => panic!("This can't happen"),
        };
        ui.painter().text(
            table_org + vec2(26., 99.).scl() + vec2(100. * i as f32, 0.).scl(),
            Align2::CENTER_CENTER,
            number.to_string(),
            FontId::proportional(scl(20.)),
            hex_to_egui_rgb(SKY_GREEN),
        );
    }

    // Draw right-side strings literals
    for i in 0..6 {
        let symbol = match i {
            0 | 5 => 'e',
            1 => 'a',
            2 => 'd',
            3 => 'g',
            4 => 'b',
            _ => panic!("This can't happen"),
        };
        ui.painter().text(
            table_org + vec2(table_width + scl(17.), scl(i as f32 * 39.)),
            Align2::CENTER_CENTER,
            symbol,
            FontId::proportional(scl(20.)),
            hex_to_egui_rgb(DARK_GREEN),
        );
    }

    // ----- Draw a hover widget -------------------------------------------- //

    let rect = Rect::from_center_size(
        imaginary_coord(hover_xy, table_org),
        vec2(scl(11.), scl(11.)),
    );

    let response = ui
        .child_ui(rect, Layout::default())
        .allocate_response(rect.size(), Sense::click());

    let visuals = ui.style().interact(&response);

    ui.painter().circle(
        imaginary_coord(hover_xy, table_org),
        scl(11.),
        visuals.bg_fill,
        visuals.bg_stroke,
    );

    response.on_hover_ui(|ui| {
        ui.label(hover_string);
    })
}

/// It receives only Direction::LeftToRight or Direction::RightToLeft params!
pub fn connect(ui: &mut egui::Ui, a: Rect, b: Rect, direction: Direction) {
    let a_org = a.center();
    let b_org = b.center();

    let mid = match direction {
        Direction::LeftToRight => {
            pos2(a.right() + scl(15.), b.top() + scl(40.))
        }
        Direction::RightToLeft => pos2(a.left() - scl(70.), b.top() + scl(80.)),
        Direction::TopDown => unreachable!(),
        Direction::BottomUp => unreachable!(),
    };

    let points = bezier_curve(a_org, mid, b_org);

    let dashed = egui::Shape::dashed_line(
        &points,
        Stroke::new(scl(4.), hex_to_egui_rgb(MID_GREEN)),
        scl(8.),
        scl(8.),
    );
    ui.painter().add(dashed);
}

// ----- Widget adapter functions ------------------------------------------- //

pub fn start_block<'a>(star: &'a Shape) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| draw_start_block(ui, star)
}

pub fn guitar_block(
    data: (&'static str, u8, u8, u8, u8),
    hover_xy: (u8, u8),
    hover_string: &'static str,
) -> impl egui::Widget {
    move |ui: &mut egui::Ui| draw_guitar_block(ui, data, hover_xy, hover_string)
}

// ----- Helper functions --------------------------------------------------- //

fn imaginary_coord(hover_xy: (u8, u8), table_org: egui::Pos2) -> egui::Pos2 {
    egui::Pos2 {
        x: table_org.x + scl(34.) + scl(50.) * hover_xy.0 as f32,
        y: table_org.y + scl(40.) * hover_xy.1 as f32,
    }
}

fn interpolate(from: f32, to: f32, perc: f32) -> f32 {
    let difference = to - from;
    from + difference * perc
}

fn bezier_curve(origin: Pos2, mid: Pos2, end: Pos2) -> Vec<Pos2> {
    let mut vec = Vec::new();
    for i in 0..100 {
        let perc = i as f32 / 100.;
        let xa = interpolate(origin.x, mid.x, perc);
        let ya = interpolate(origin.y, mid.y, perc);
        let xb = interpolate(mid.x, end.x, perc);
        let yb = interpolate(mid.y, end.y, perc);

        // Target position
        let x = interpolate(xa, xb, perc);
        let y = interpolate(ya, yb, perc);

        vec.push(pos2(x, y));
    }
    vec
}
