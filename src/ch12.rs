use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{Cube, Plane, Sphere};
use crate::domain::pattern::Pattern;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let example = 2;
    println!("Running ch12... (example #{})", example);

    println!("Progress...");
    println!("|----------|");
    print!(" ");

    let (world, camera) = match example {
        1 => build_example_1()?,
        2 => build_example_2()?,
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
    crate::utils::write_imagefile("refractive_scene.ppm", "/tmp", &canvas)
}

fn build_example_2() -> Result<(World, Camera), Error> {
    // camera
    let camera_width = 600;
    let camera_height = 600;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(10.0, 5.0, -5.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));

    // wall
    let wall_transform = Matrix::new_translation(0.0, -10.0, 0.0); // * &Matrix::new_rotation_x(PI/);
    let wall = Plane::new()
        .transformation(wall_transform)
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
        .build();

    let glass_ball = Sphere::new()
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .transparency(0.9)
                .substance(Substance::GLASS)
                .build(),
        )
        .transformation(Matrix::new_translation(1.0, 0.0, 0.0))
        .build();

    let glass_cube = Cube::new()
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .transparency(0.9)
                .substance(Substance::GLASS)
                .build(),
        )
        .transformation(Matrix::new_translation(-1.0, 0.0, 0.0))
        .build();

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![wall.into(), glass_ball.into(), glass_cube.into()].as_mut());

    Result::Ok((world, camera))
}
fn build_example_1() -> Result<(World, Camera), Error> {
    // camera
    let camera_width = 600;
    let camera_height = 600;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(0.0, 0.0, -5.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));

    // wall
    let wall_transform = &Matrix::new_translation(0.0, 0.0, 10.0) * &Matrix::new_rotation_x(1.5708);
    let wall = Plane::new()
        .transformation(wall_transform)
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
        .build();

    let glass_ball = Sphere::new()
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .transparency(0.9)
                .substance(Substance::GLASS)
                .build(),
        )
        .build();

    let hollow_center = Cube::new()
        .transformation(Matrix::new_scaling(0.5, 0.5, 0.5))
        .material(
            Material::new()
                .color(Color::new(1.0, 1.0, 1.0))
                .ambient(0.0)
                .diffuse(0.0)
                .specular(0.9)
                .shininess(300.0)
                .reflective(0.9)
                .transparency(0.9)
                .substance(Substance::AIR)
                .build(),
        )
        .build();

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![wall.into(), glass_ball.into(), hollow_center.into()].as_mut());

    Result::Ok((world, camera))
}
