use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{
    Cone, Cube, Cylinder, CylinderBuilder, Group, GroupBuilder, Object, Plane, Sphere,
    SphereBuilder,
};
use crate::domain::pattern::Pattern;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let example = 1;
    println!("Running ch14... (example #{})", example);

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
    crate::utils::write_imagefile("group_scene.ppm", "/tmp", &canvas)
}

fn build_example_1<'a>() -> Result<(World<'a>, Camera), Error> {
    // camera
    let camera_width = 1200;
    let camera_height = 1200;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(10.0, 7.0, -10.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));

    // hexagon
    let hexagon = build_hexagon();

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world.objects.append(vec![hexagon].as_mut());

    Result::Ok((world, camera))
}

fn build_hexagon<'a>() -> Object<'a> {
    let mut hex = Group::new();

    for n in 0..6 {
        let side = build_hexagon_side()
            .transformation(Matrix::new_rotation_y(n as f64 * PI / 3.0))
            .build();
        hex = hex.add_child(side.into());
    }

    hex.build().into()
}

fn build_hexagon_corner() -> SphereBuilder {
    Sphere::new().transformation(
        &Matrix::new_translation(0.0, 0.0, -1.0) * &Matrix::new_scaling(0.25, 0.25, 0.25),
    )
}

fn build_hexagon_edge() -> CylinderBuilder {
    let transformation = &(&(&Matrix::new_translation(0.0, 0.0, -1.0)
        * &Matrix::new_rotation_y(-PI / 6.0))
        * &Matrix::new_rotation_z(-PI / 2.0))
        * &Matrix::new_scaling(0.25, 1.0, 0.25);

    Cylinder::new()
        .minimum(0.0)
        .maximum(1.0)
        .transformation(transformation)
}

fn build_hexagon_side<'a>() -> GroupBuilder<'a> {
    let corner = build_hexagon_corner().build();
    let edge = build_hexagon_edge().build();

    Group::new().add_child(corner.into()).add_child(edge.into())
}
