use crate::domain::camera::Camera;
use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersection, Intersections};
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::world::World;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

#[test]
fn test1_world_can_be_constructed() {
    let w = World::new();
    assert!(w.objects.is_empty());
    assert_eq!(w.light_source, Option::None);
}

fn build_test_world() -> World {
    let light_point = Point::new(-10.0, 10.0, -10.0);
    let intensity = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(light_point, intensity);

    let mc = Color::new(0.8, 1.0, 0.6);
    let m1 = Material::new_full(
        mc,
        Material::DEFAULT_AMBIENT,
        0.7,
        0.2,
        Material::DEFAULT_SHININESS,
    );
    let s1 = Sphere::new_material(m1);

    let t2 = Matrix::new_scaling(0.5, 0.5, 0.5);
    let s2 = Sphere::new(t2);

    let mut w = World::new();
    w.light_source = Some(light);
    w.add_object(s1);
    w.add_object(s2);

    w
}

#[test]
fn test2_validate_default_world() {
    let w = build_test_world();
    assert!(w.light_source.as_ref().is_some());
    assert_eq!(
        w.light_source.as_ref().unwrap().position,
        Point::new(-10.0, 10.0, -10.0)
    );
    assert_eq!(
        w.light_source.as_ref().unwrap().intensity,
        Color::new(1.0, 1.0, 1.0)
    );

    let mc = Color::new(0.8, 1.0, 0.6);
    let m1 = Material::new_full(
        mc,
        Material::DEFAULT_AMBIENT,
        0.7,
        0.2,
        Material::DEFAULT_SHININESS,
    );
    let s1_exp = Sphere::new_material(m1);
    assert!(w.objects.contains(&s1_exp));

    let t2 = Matrix::new_scaling(0.5, 0.5, 0.5);
    let s2_exp = Sphere::new(t2);
    assert!(w.objects.contains(&s2_exp));
}

#[test]
fn test3_intersect_world_with_ray() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut xs: Intersections = w.intersect(&r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs.hit().unwrap().distance, 4.0);
    assert_eq!(xs.hit().unwrap().distance, 4.5);
    assert_eq!(xs.hit().unwrap().distance, 5.5);
    assert_eq!(xs.hit().unwrap().distance, 6.0);
}

#[test]
fn test4_precomputing_state_of_intersection() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::new_unit();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
    assert_eq!(comps.distance, i.distance);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
    assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn test5_prepare_computations_when_hit_outside_and_inside() {
    // outside hit
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::new_unit();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
    assert_eq!(comps.inside, false);

    // inside hit
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let i = Intersection::new(1.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
    assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
    assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn test6_shading_an_intersection_inside_and_outside() {
    // outside
    let mut w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: &Sphere = w.objects.first().unwrap();
    let i = Intersection::new(4.0, shape);
    let comps = Computations::prepare_computations(&i, &r);
    let c: Color = w.shade_hit(&comps);
    let c_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(c, c_exp);

    // inside
    w.light_source = Some(Light::new(Point::new(0.0, 0.25, 0.0), Color::WHITE));
    let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects.get(1).unwrap();
    let i = Intersection::new(0.5, shape);
    let comps = Computations::prepare_computations(&i, &r);
    let c = w.shade_hit(&comps);
    let c_exp = Color::new(0.90498, 0.90498, 0.90498);
    assert_eq!(c, c_exp);
}

#[test]
fn test7_color_when_ray_misses() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
    let c = w.color_at(&r);
    let c_exp = Color::BLACK;
    assert_eq!(c, c_exp);
}

#[test]
fn test8_color_when_ray_hits() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(&r);
    let c_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(c, c_exp);
}

#[test]
fn test9_color_intersection_behind_ray() {
    let mut w = build_test_world();
    w.objects[0].material.ambient = 1.0;
    w.objects[1].material.ambient = 1.0;
    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    let c = w.color_at(&r);
    let c_exp = w.objects[1].material.color;
    assert_eq!(c, c_exp);
}

#[test]
fn test10_view_transformation_matrix_for_default_orientation() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, -1.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::new_view_transformation(&from, &to, &up);
    let t_exp = crate::domain::matrix::IDENTITY.clone();
    assert_eq!(t, t_exp);
}

#[test]
fn test11_view_transformation_matrix_positive_z_direction() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, 1.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::new_view_transformation(&from, &to, &up);
    let t_exp = Matrix::new_scaling(-1.0, 1.0, -1.0);
    assert_eq!(t, t_exp);
}

#[test]
fn test12_view_transformation_moves_world() {
    let from = Point::new(0.0, 0.0, 8.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let t = Matrix::new_view_transformation(&from, &to, &up);
    let t_exp = Matrix::new_translation(0.0, 0.0, -8.0);
    assert_eq!(t, t_exp);
}

#[test]
#[rustfmt::skip::macros(vec)]
fn test13_view_transformation_arbitrary() {
    let from = Point::new(1.0, 3.0, 2.0);
    let to = Point::new(4.0, -2.0, 8.0);
    let up = Vector::new(1.0, 1.0, 0.0);
    let t = Matrix::new_view_transformation(&from, &to, &up);
    let t_exp = Matrix::new(
        4,
        4,
        vec![-0.50709, 0.50709, 0.67612, -2.36643,
             0.76772, 0.60609, 0.12122, -2.82843,
             -0.35857, 0.59761, -0.71714, 0.0,
             0.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(t, t_exp);
}

#[test]
fn test14_construct_camera() {
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
fn test15_pixel_size_for_canvas() {
    // horizontal
    let c = Camera::new(200, 125, PI / 2.0);
    assert!(crate::domain::epsilon_eq(c.pixel_size, 0.01));

    // vertical
    let c = Camera::new(125, 200, PI / 2.0);
    assert!(crate::domain::epsilon_eq(c.pixel_size, 0.01));
}

#[test]
fn test16_constructing_ray_through_camera_canvas() {
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
fn test17_rendering_world_with_camera() {
    let w = build_test_world();
    let mut c = Camera::new(11, 11, PI / 2.0);
    let from = Point::new(0.0, 0.0, -5.0);
    let to = Point::ORIGIN;
    let up = Vector::new(0.0, 1.0, 0.0);
    c.transform = Matrix::new_view_transformation(&from, &to, &up);
    let image = w.render(&c);
    let color_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(image[5][5], color_exp);
}
