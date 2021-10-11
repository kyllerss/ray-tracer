use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::matrix::Matrix;
use crate::domain::{Point, Vector};
use crate::tests::world::build_test_world;
use std::f64::consts::PI;

#[test]
fn ch7_test14_construct_camera() {
    let hsize = 160;
    let vsize = 120;
    let field_of_view = PI / 2.0;
    let c = Camera::new(hsize, vsize, field_of_view);
    assert_eq!(c.hsize, hsize);
    assert_eq!(c.vsize, vsize);
    assert_eq!(c.field_of_view, field_of_view);
    assert_eq!(c.transform, crate::domain::matrix::IDENTITY.clone());
}

#[test]
fn ch7_test15_pixel_size_for_canvas() {
    // horizontal
    let c = Camera::new(200, 125, PI / 2.0);
    assert!(crate::domain::epsilon_eq(c.pixel_size, 0.01));

    // vertical
    let c = Camera::new(125, 200, PI / 2.0);
    assert!(crate::domain::epsilon_eq(c.pixel_size, 0.01));
}

#[test]
fn ch7_test16_constructing_ray_through_camera_canvas() {
    // ray through center of canvas
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, Point::ORIGIN);
    assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));

    // ray through corner of canvas
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
    assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));

    // ray when camera is transformed
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.transform = &Matrix::new_rotation_y(PI / 4.0) * &Matrix::new_translation(0.0, -2.0, 5.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
    assert_eq!(
        r.direction,
        Vector::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
    );
}

#[test]
fn ch7_test17_rendering_world_with_camera() {
    let w = build_test_world();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = Point::new(0.0, 0.0, -5.0);
    let to = Point::ORIGIN;
    let up = Vector::new(0.0, 1.0, 0.0);
    c.transform = Matrix::new_view_transformation(&from, &to, &up);
    let image = w.render(&c, &|_itr, _total_size| {});
    let color_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(image[5][5], color_exp);
}
