use egui::{pos2, Color32, Pos2, Shape, Stroke};

pub fn draw_star(
    origin: Pos2,
    num_points: u32,
    radius: f32,
    color: Color32,
) -> Shape {
    let mut points = vec![];
    let deg_to_rad = 3.14159 / 180.;
    let mut count = 0;
    let mut i = 0;

    while i <= num_points * 2 {
        let deg_in_rad = i as f32 * 360. / (num_points * 2) as f32 * deg_to_rad;
        if count % 2 != 0 {
            points.push(pos2(
                origin.x + deg_in_rad.cos() * radius,
                origin.y + deg_in_rad.sin() * radius,
            ));
        } else {
            points.push(pos2(
                origin.x + deg_in_rad.cos() * radius / 2.,
                origin.y + deg_in_rad.sin() * radius / 2.,
            ));
        }
        count += 1;
        i += 1;
    }

    Shape::convex_polygon(points, color, Stroke::NONE)
}
