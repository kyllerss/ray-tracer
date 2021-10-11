use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};

pub fn run() -> Result<(), Error> {
    println!("Running ch7...");

    // floor
    let mut floor = Sphere::new(Matrix::new_scaling(10.0, 0.01, 10.0));
    floor.material = Material::new_full(
        Color::new(1.0, 0.9, 0.9),
        Material::DEFAULT_AMBIENT,
        Material::DEFAULT_DIFFUSE,
        0.0,
        Material::DEFAULT_SHININESS,
    );

    // left wall
    let mut left_wall = Sphere::new_unit();
    left_wall.transformation = &(&(&Matrix::new_translation(0.0, 0.0, 5.0)
        * &Matrix::new_rotation_y(-PI / 4.0))
        * &Matrix::new_rotation_x(PI / 2.0))
        * &Matrix::new_scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material.clone();

    // right wall
    let mut right_wall = Sphere::new_unit();
    right_wall.transformation = &(&(&Matrix::new_translation(0.0, 0.0, 5.0)
        * &Matrix::new_rotation_y(PI / 4.0))
        * &Matrix::new_rotation_x(PI / 2.0))
        * &Matrix::new_scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material.clone();

    // middle sphere
    let mut middle = Sphere::new_unit();
    middle.transformation = Matrix::new_translation(-0.5, 1.0, 0.5);
    middle.material = Material::new_full(
        Color::new(0.1, 1.0, 0.5),
        Material::DEFAULT_AMBIENT,
        0.7,
        0.3,
        Material::DEFAULT_SHININESS,
    );

    // right sphere
    let mut right = Sphere::new_unit();
    right.transformation =
        &Matrix::new_translation(1.5, 0.5, -0.5) * &Matrix::new_scaling(0.5, 0.5, 0.5);
    right.material = Material::new_full(
        Color::new(0.5, 1.0, 0.1),
        Material::DEFAULT_AMBIENT,
        0.7,
        0.3,
        Material::DEFAULT_SHININESS,
    );

    // left sphere
    let mut left = Sphere::new_unit();
    left.transformation =
        &Matrix::new_translation(-1.5, 0.33, -0.75) * &Matrix::new_scaling(0.33, 0.33, 0.33);
    left.material = Material::new_full(
        Color::new(1.0, 0.8, 0.1),
        Material::DEFAULT_AMBIENT,
        0.7,
        0.3,
        Material::DEFAULT_SHININESS,
    );

    // world
    let light_source = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut world = World::new();
    world.light_source = Some(light_source);
    world
        .objects
        .append(vec![floor, left_wall, right_wall, middle, left, right].as_mut());

    // camera
    let scale = 16;
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
    let canvas = world.render(&camera, &|itr: usize, total_size: usize| {
        if ((itr as f64 / total_size as f64) * 100.0) % 10.0 == 0.0 {
            print!("#");
            let _ = stdout().flush();
        }
    });

    //canvas.invert_y();
    println!("{}", "");
    println!("Rendering to file...");
    crate::utils::write_imagefile("spheres_scene.ppm", "/tmp", &canvas)
}