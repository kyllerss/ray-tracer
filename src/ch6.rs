use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::object::Object;
use crate::domain::ray::Ray;
use crate::domain::Point;
use std::io::{stdout, Error, Write};

pub fn run() -> Result<(), Error> {
    println!("Running ch6...");

    // world
    let wall_z = 10.0;

    // viewer
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    // sphere object
    let mut sphere = Object::new_sphere_unit();
    let material = Material::new().color(Color::new(1.0, 0.2, 1.0)).build();
    sphere.shape_mut().material = material;

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::WHITE;
    let light = Light::new(light_position, light_color);

    let canvas_length: usize = 400;
    let mut canvas = Canvas::new(canvas_length, canvas_length, Color::default());

    println!("Progress...");
    println!("|----------|");
    print!(" ");
    let mut iteration: f64 = 0.0;

    let wall_width = 7.0; // TODO make bigger (to test impact)
    let half = wall_width as f64 / 2.0;
    let pixel_size = wall_width / canvas_length as f64;
    for x in 0..canvas_length {
        for y in 0..canvas_length {
            // point to cast a ray to is
            let x_wall_point = (x as f64 * pixel_size) - half;
            let y_wall_point = (y as f64 * pixel_size) - half;
            let wall_point = Point::new(x_wall_point, y_wall_point, wall_z);
            let wall_pixel_vector = (&wall_point - &ray_origin).normalize();

            let ray = Ray::new(ray_origin, wall_pixel_vector);
            let mut intersections = sphere.intersect(&ray);

            if !intersections.is_empty() {
                let render_point = Point::new(x as f64, y as f64, wall_z);

                let intersection = intersections.hit_unchecked().unwrap();
                let object = intersection.object;
                let point = ray.position(intersection.distance);
                let normal = object.normal_at(&point);
                let eye = -ray.direction;

                let intersection_color = Light::lighting(
                    &object.shape().material,
                    &object,
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                );
                canvas.render(
                    render_point.x().round() as usize,
                    render_point.y().round() as usize,
                    intersection_color,
                );
            }

            // report progress
            iteration += 1.0;
            if ((iteration / (canvas_length as f64 * canvas_length as f64)) * 100.0) % 10.0 == 0.0 {
                print!("#");
                let _ = stdout().flush();
            }
        }
    }
    println!("{}", "");
    println!("Rendering to file...");

    canvas.invert_y();
    crate::utils::write_imagefile("sphere.ppm", "/tmp", &canvas)
}
