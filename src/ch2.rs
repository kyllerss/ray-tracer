use crate::ch1::Tick;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::{Point, Vector};
use crate::utils::image_writer::{Format, ImageWriter};
use std::fs;
use std::time::SystemTime;

pub fn run() {
    println!("Running ch2...");

    let mut c = Canvas::new(10, 5, Color::default());
    let mut tick = Tick {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
        projectile: Point::new(0.0, 1.0, 0.0),
        projectile_velocity: Vector::new(1.0, 1.8, 0.0) * 11.25,
    };
    let projectile_color = Color::new(1.0, 0.0, 0.0);

    let mut projectile = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(1.0, 1.0, 0.0);
    for _i in 0..400 {
        //println!("x:{}, y:{}", tick.projectile.x(), tick.projectile.y());
        //crate::ch1::apply_tick(&mut tick);
        //c.render(tick.projectile, projectile_color);
        projectile = projectile + direction;
        c.render(projectile, projectile_color);
    }

    let writer = ImageWriter::new(Format::Ppm3, &c);
    let ppm = writer.to_string();
    let timestamp = {
        let start = SystemTime::now();
        start.duration_since(SystemTime::UNIX_EPOCH)
    }
    .expect("Unable to calculate system time.")
    .as_millis();
    let filename = format!("/tmp/ray-tracer/ch2_image_{}.ppm", timestamp);
    fs::write(filename, ppm).expect("Unable to write file.");
}
