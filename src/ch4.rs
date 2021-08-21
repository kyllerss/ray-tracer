use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::matrix::Matrix;
use crate::domain::Point;
use std::f64::consts::PI;
use std::io::Error;

pub fn run() -> Result<(), Error> {
    println!("Running ch4...");

    let mut canvas = Canvas::new(100, 100, Color::default());

    let radius = 100.0 * 3.0 / 8.0;
    let center_point = 50.0;
    let oclock = Point::new(0.0, 1.0, 0.0);

    for hour in 0..12 {
        // Rotating along z-axis has it rotating counter-clockwise.
        // Therefore, subtracting against 12 to simulate clockwise rotation.
        let hour_rotation = Matrix::new_rotation_z((12 - hour) as f64 * PI / 6.0);
        let hour_point = (&hour_rotation * &oclock)
            .mult_x(radius)
            .mult_y(radius)
            .add_x(center_point)
            .add_y(center_point);

        // Fades to black to help visualize order of render
        let color_comp = 1.0 - (hour as f32 * 1.0 / 12.0);
        let c = Color::new(color_comp, color_comp, color_comp);
        canvas.render(hour_point, c);
    }

    canvas.invert_y();
    crate::utils::write_imagefile("clock.ppm", "/tmp", &canvas)
}
