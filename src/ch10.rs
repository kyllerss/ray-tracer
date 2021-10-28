use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Plane, Sphere};
use crate::domain::pattern::Pattern;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    println!("Running ch10...");

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
        .build();

    // right wall
    let right_wall_pattern = Pattern::new_checkered(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 0.0, 1.0),
        crate::domain::matrix::IDENTITY.clone(),
    );

    let t = &Matrix::new_translation(15.0, 0.0, 5.0)
        * &(&Matrix::new_rotation_z(PI / 2.0) * &Matrix::new_rotation_y(1.5 * PI / 4.0));
    let mut right_wall: Object = Plane::new().transformation(t).build().into();
    right_wall.shape_mut().material = Material::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .pattern(right_wall_pattern)
        .build();

    // left wall
    let left_wall_pattern = Pattern::new_ringed(
        Color::WHITE,
        Color::BLACK,
        Matrix::new_scaling(0.5, 0.5, 0.5),
    );

    let t = &Matrix::new_translation(-15.0, 0.0, 5.0)
        * &(&Matrix::new_rotation_z(PI / 2.0) * &Matrix::new_rotation_y(4.5 * PI / 4.0));
    let mut left_wall: Object = Plane::new().transformation(t).build().into();
    left_wall.shape_mut().material = Material::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .pattern(left_wall_pattern)
        .build();

    // middle sphere
    let middle_pattern = Pattern::new_checkered(
        Color::new(0.2, 0.8, 0.2),
        Color::new(0.8, 0.2, 0.8),
        crate::domain::matrix::IDENTITY.clone(),
    );
    let mut middle: Object = Sphere::new().build().into();
    middle.shape_mut().transformation =
        &Matrix::new_translation(-0.5, 1.0, 0.5) * &Matrix::new_rotation_x(PI / 4.0);
    middle.shape_mut().material = Material::new()
        .color(Color::new(0.1, 1.0, 0.5))
        .diffuse(0.7)
        .specular(0.3)
        .pattern(middle_pattern)
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
        .append(vec![floor, left_wall, right_wall, middle, left, right].as_mut());

    // camera
    let scale = 8;
    let camera_width = 100 * scale;
    let camera_height = 50 * scale;
    let mut camera = Camera::new(camera_width, camera_height, PI / 3.0);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(0.0, 1.5, -10.0),
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
