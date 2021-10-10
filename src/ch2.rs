use crate::ch1::Tick;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::{Point, Vector};
use std::io::Error;

pub fn run() -> Result<(), Error> {
    println!("Running ch2...");

    let mut c = Canvas::new(900, 550, Color::default());
    let mut tick = Tick {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
        projectile: Point::new(0.0, 1.0, 0.0),
        projectile_velocity: &Vector::new(1.0, 1.8, 0.0) * 5.25,
        count: 0.0,
    };
    let projectile_color = Color::new(1.0, 0.0, 0.0);

    //let projectile = Point::new(0.0, 0.0, 0.0);
    //let direction = Vector::new(1.0, 1.0, 0.0);
    while tick.projectile.y() > 0.0 {
        crate::ch1::apply_tick(&mut tick);
        println!("x:{}, y:{}", tick.projectile.x(), tick.projectile.y());
        c.render(
            tick.projectile.x().round() as usize,
            tick.projectile.y().round() as usize,
            projectile_color,
        );
        // projectile = projectile + direction;
        // c.render(projectile, projectile_color);
    }
    // println!("---------------");
    // for (i, pixel) in c.into_iter().enumerate() {
    //     if i % 10 == 0 {
    //         println!("")
    //     };
    //     let hit = *pixel == projectile_color;
    //     print!("{}", if hit { 1 } else { 0 });
    // }
    // println!("");
    c.invert_y();
    crate::utils::write_imagefile("ch2_projectile.ppm", "/tmp", &c)
}
