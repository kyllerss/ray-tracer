use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Plane, Sphere};
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    println!("Running ch9...");

    // floor
    //let mut t = &Matrix::new_rotation_z(PI / 2.0) * &Matrix::new_translation(0.0, 0.0, 3.0);
    let t = Matrix::new_rotation_z(PI / 3.0);
    let floor: Object = Plane::builder().transformation(t).build().into();
    // Matrix::new_scaling(10.0, 0.01, 10.0)
    // floor.shape_mut().material = Material::new_full(
    //     Color::new(1.0, 0.9, 0.9),
    //     Material::DEFAULT_AMBIENT,
    //     Material::DEFAULT_DIFFUSE,
    //     0.0,
    //     Material::DEFAULT_SHININESS,
    // );

    // middle sphere
    let middle: Object = Sphere::builder()
        .transformation(Matrix::new_translation(-0.5, 1.0, 0.5))
        .material(
            Material::new()
                .color(Color::new(0.1, 1.0, 0.5))
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        )
        .build()
        .into();

    // right sphere
    let right: Object = Sphere::builder()
        .transformation(
            &Matrix::new_translation(1.5, 0.5, -0.5) * &Matrix::new_scaling(0.5, 0.5, 0.5),
        )
        .material(
            Material::new()
                .color(Color::new(0.5, 1.0, 0.1))
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        )
        .build()
        .into();

    // left sphere
    let left: Object = Sphere::builder()
        .transformation(
            &Matrix::new_translation(-1.5, 0.33, -0.75) * &Matrix::new_scaling(0.33, 0.33, 0.33),
        )
        .material(
            Material::new()
                .color(Color::new(1.0, 0.8, 0.1))
                .diffuse(0.7)
                .specular(0.3)
                .build(),
        )
        .build()
        .into();

    // world
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![floor, middle, left, right].as_mut());

    // camera
    let scale = 2;
    let camera_width = 100 * scale;
    let camera_height = 50 * scale;
    let mut camera = Camera::new(camera_width, camera_height, PI / 3.0);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(0.0, 1.5, -5.0),
        &Point::new(0.0, 1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    println!("Progress...");
    println!("|----------|");
    print!(" ");

    // canvas
    let canvas = world.render(
        &camera,
        Arc::new(move |itr: usize, total_size: usize| {
            if ((itr as f64 / total_size as f64) * 100.0) % 10.0 == 0.0 {
                print!("#");
                let _ = stdout().flush();
            }
        }),
    );

    //canvas.invert_y();
    println!("{}", "");
    println!("Rendering to file...");
    crate::utils::write_imagefile("spheres_scene.ppm", "/tmp", &canvas)
}
