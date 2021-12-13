use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{Cube, Cylinder, Plane, Sphere};
use crate::domain::pattern::Pattern;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let example = 1;
    println!("Running ch13... (example #{})", example);

    println!("Progress...");
    println!("|----------|");
    print!(" ");

    let (world, camera) = match example {
        1 => build_example_1()?,
        _ => panic!("Unknown example: {}", example),
    };

    // render to canvas
    let canvas = world.render(
        &camera,
        Arc::new(move |itr: usize, total_size: usize| {
            if ((itr as f64 / total_size as f64) * 100.0) % 10.0 == 0.0 {
                print!("#");
                let _ = stdout().flush();
            }
        }),
    );

    println!("{}", "");
    println!("Rendering to file...");
    crate::utils::write_imagefile("cylinders_cones_scene.ppm", "/tmp", &canvas)
}

fn build_example_1() -> Result<(World, Camera), Error> {
    // camera
    let camera_width = 300;
    let camera_height = 300;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(10.0, 7.0, -10.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));

    // floor
    let floor = Plane::new()
        .material(
            Material::new()
                .pattern(Pattern::new_checkered(
                    Color::new(0.15, 0.15, 0.15),
                    Color::new(0.85, 0.85, 0.85),
                    crate::domain::matrix::IDENTITY.clone(),
                ))
                .ambient(0.8)
                .diffuse(0.2)
                .specular(0.0)
                .build(),
        )
        .transformation(Matrix::new_translation(0.0, -2.0, 0.0))
        .build();

    let cylinder = Cylinder::new()
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .build(),
        )
        .minimum(0.0)
        .maximum(2.0)
        .closed(true)
        .transformation(
            &Matrix::new_translation(0.75, 0.0, 0.0) * &Matrix::new_rotation_x(-PI / 8.0),
        )
        .build();

    let ball = Sphere::new()
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .substance(Substance::GLASS)
                .build(),
        )
        .transformation(Matrix::new_translation(3.0, 0.0, 0.0))
        .build();

    let cube = Cube::new()
        .material(
            Material::new()
                .color(Color::new(0.7, 0.2, 0.0))
                .ambient(0.1)
                .diffuse(0.5)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.1)
                //.transparency(0.9)
                //.substance(Substance::GLASS)
                .build(),
        )
        .transformation(
            &Matrix::new_translation(-3.0, 0.0, 0.0) * &Matrix::new_rotation_y(PI / 3.0),
        )
        .build();

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![floor.into(), ball.into(), cube.into(), cylinder.into()].as_mut());

    Result::Ok((world, camera))
}
