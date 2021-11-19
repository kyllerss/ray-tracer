use crate::domain::camera::Camera;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Plane, Sphere};
use crate::domain::pattern::Pattern;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use log::info;
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let example = 4;

    info!("Running ch11... (example #{})", example);

    info!("Progress...");

    let (world, camera) = match example {
        1 => build_example_1()?,
        2 => build_example_2()?,
        3 => build_example_3()?,
        4 => build_example_4()?,
        _ => panic!("Unknown example: {}", example),
    };

    // render to canvas
    let canvas = world.render(
        &camera,
        Arc::new(move |itr: usize, total_size: usize| {
            if ((itr as f64 / total_size as f64) * 100.0) % 10.0 == 0.0 {
                info!("{}/{}", itr, total_size);
            }
        }),
    );

    info!("{}", "");
    info!("Rendering to file...");
    crate::utils::write_imagefile("refractive_scene.ppm", "/tmp", &canvas)
}

fn build_example_4() -> Result<(World, Camera), Error> {
    // camera
    let camera_width = 300;
    let camera_height = 300;
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
                .reflective(0.5)
                .transparency(0.9)
                .substance(Substance::GLASS)
                .build(),
        )
        .build();

    // let hollow_center = Sphere::new()
    //     .transformation(Matrix::new_scaling(0.5, 0.5, 0.5))
    //     .material(
    //         Material::new()
    //             .color(Color::new(1.0, 1.0, 1.0))
    //             .ambient(0.0)
    //             .diffuse(0.0)
    //             .specular(0.9)
    //             .shininess(300.0)
    //             .reflective(0.9)
    //             .transparency(0.9)
    //             .substance(Substance::AIR)
    //             .build(),
    //     )
    //     .build();

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world.objects.append(
        vec![
            wall.into(),
            glass_ball.into(), /*, hollow_center.into()*/
        ]
        .as_mut(),
    );

    Result::Ok((world, camera))
}

fn build_example_3() -> Result<(World, Camera), Error> {
    // camera
    let camera_width = 600;
    let camera_height = 600;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(0.0, -2.0, -0.5),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(5.0, -7.0, -15.0), Color::new(0.9, 0.9, 0.9));

    // wall
    let wall_transform = &Matrix::new_translation(0.0, 0.0, 1.5) * &Matrix::new_rotation_x(1.5708);
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
                //.ambient(0.0)
                //.diffuse(0.0)
                //.specular(0.9)
                //.shininess(300.0)
                //.reflective(0.1)
                .transparency(1.0)
                .substance(Substance::GLASS)
                .build(),
        )
        .build();

    let hollow_center = Sphere::new()
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
    world.objects.append(
        vec![
            wall.into(),
            glass_ball.into(), /*, hollow_center.into()*/
        ]
        .as_mut(),
    );

    Result::Ok((world, camera))
}

fn build_example_2() -> Result<(World, Camera), Error> {
    let floor = Plane::new()
        .material(
            Material::new()
                .pattern(Pattern::new_checkered(
                    Color::WHITE,
                    Color::BLACK,
                    crate::domain::matrix::IDENTITY.clone(),
                ))
                .build(),
        )
        .build();

    let sphere = Sphere::new()
        .material(
            Material::new()
                .substance(Substance::GLASS)
                .transparency(0.9)
                .build(),
        )
        .transformation(Matrix::new_scaling(1.5, 1.5, 1.5))
        .build();

    let inner_sphere = Sphere::new()
        .material(
            Material::new()
                .substance(Substance::AIR)
                .transparency(0.9)
                .build(),
        )
        .transformation(Matrix::new_scaling(0.9, 0.9, 0.9))
        .build();

    // world
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Color::WHITE);
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![floor.into(), sphere.into(), inner_sphere.into()].as_mut());

    // camera
    let scale = 4;
    let camera_width = 100 * scale;
    let camera_height = 50 * scale;
    let mut camera = Camera::new(camera_width, camera_height, PI / 3.0);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(0.0, 2.0, 0.0),
        &Point::new(0.0, -1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    Result::Ok((world, camera))
}

fn build_example_1() -> Result<(World, Camera), Error> {
    // floor
    let floor_pattern = Pattern::new_striped(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 0.0, 1.0),
        crate::domain::matrix::IDENTITY.clone(),
    );

    let t = crate::domain::matrix::IDENTITY.clone(); //Matrix::new_rotation_z(PI / 3.0);
    let mut floor: Object = Plane::new().transformation(t).build().into();
    floor.shape_mut().material = Material::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .pattern(floor_pattern)
        .reflective(0.2)
        .build();

    // back wall
    let t = &Matrix::new_translation(0.0, 0.0, 5.0) * &Matrix::new_rotation_x(PI / 2.0);
    let mut right_wall: Object = Plane::new().transformation(t).build().into();
    right_wall.shape_mut().material = Material::new()
        .color(Color::BLACK)
        .specular(0.0)
        .reflective(0.9)
        .shininess(100.0)
        .build();

    // middle sphere
    let mut middle: Object = Sphere::new().build().into();
    middle.shape_mut().transformation =
        &Matrix::new_translation(-0.5, 1.0, 0.5) * &Matrix::new_rotation_x(PI / 4.0);
    middle.shape_mut().material = Material::new()
        .substance(Substance::GLASS)
        .transparency(1.0)
        .build();

    // right sphere
    let right_pattern = Pattern::new_ringed(
        Color::WHITE,
        Color::BLACK,
        &Matrix::new_scaling(0.1, 0.1, 0.1)
            * &(&Matrix::new_rotation_x(PI / 4.0) * &Matrix::new_rotation_y(PI / 4.0)),
    );
    let mut right: Object = Sphere::new().build().into();
    right.shape_mut().transformation =
        &Matrix::new_translation(2.0, 0.5, -0.5) * &Matrix::new_scaling(0.5, 0.5, 0.5);
    right.shape_mut().material = Material::new()
        .diffuse(0.7)
        .specular(0.3)
        .pattern(right_pattern)
        .build();

    // left sphere
    let left_pattern = Pattern::new_striped(
        Color::new(0.33, 0.66, 0.33),
        Color::new(0.66, 0.33, 0.66),
        Matrix::new_scaling(0.33, 0.33, 0.33),
    );
    let mut left: Object = Sphere::new().build().into();
    left.shape_mut().transformation =
        &Matrix::new_translation(-2.0, 0.33, -0.75) * &Matrix::new_scaling(0.33, 0.33, 0.33);
    left.shape_mut().material = Material::new()
        .diffuse(0.7)
        .specular(0.3)
        .pattern(left_pattern)
        .build();

    // world
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![floor, right_wall, middle, left, right].as_mut());

    // camera
    let scale = 4;
    let camera_width = 100 * scale;
    let camera_height = 50 * scale;
    let mut camera = Camera::new(camera_width, camera_height, PI / 3.0);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(3.0, 2.5, -10.0),
        &Point::new(0.0, 1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    Result::Ok((world, camera))
}
