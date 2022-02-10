use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::matrix::Matrix;
use crate::domain::object::{
    Cylinder, CylinderBuilder, Group, GroupBuilder, Object, Sphere, SphereBuilder,
};
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::borrow::Borrow;
use std::f64::consts::PI;
use std::io::{stdout, Error, Write};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let example = 1;
    println!("Running ch15... (example #{})", example);

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
    crate::utils::write_imagefile("smooth_triangles_scene.ppm", "/tmp", &canvas)
}

fn build_example_1<'a>() -> Result<(World<'a>, Camera), Error> {
    // camera
    let camera_width = 300;
    let camera_height = 300;
    let mut camera = Camera::new(camera_width, camera_height, 0.45);
    camera.transform = Matrix::new_view_transformation(
        &Point::new(10.0, 7.0, -200.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    // light
    let light_source = Light::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));

    // 3d model file
    let contents =
        crate::utils::extract_contents("/home/klomeli/Downloads/3d-models/astronaut1.obj")?;
    let r = crate::utils::obj_parser::parse_obj_file(contents.as_str());
    assert!(r.is_some());

    let p = r.unwrap();
    let model = {
        let model = p.collapse_to_single_group();
        assert!(model.is_some());
        *model.unwrap()
    };

    // world
    let mut world = World::new();
    world.light_source = Some(light_source);
    world.objects.append(vec![model].as_mut());

    Result::Ok((world, camera))
}
