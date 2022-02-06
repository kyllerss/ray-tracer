use crate::domain::color::Color;
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Sphere};
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

#[test]
fn ch5_test3_ray_intersects_sphere_at_two_points() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere: Object = Sphere::builder().build().into();
    let mut xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 4.0);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 6.0);
}

#[test]
fn ch5_test4_ray_intersects_sphere_at_tangent() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere: Object = Sphere::builder().build().into();
    let mut xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 5.0);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 5.0);
}

#[test]
fn ch5_test5_ray_misses_intersection_on_sphere() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere: Object = Sphere::builder().build().into();
    let xs = sphere.intersect(&ray);
    assert!(xs.is_empty());
}

#[test]
fn ch5_test6_ray_intersects_sphere_when_ray_at_origin() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let sphere: Object = Sphere::builder().build().into();
    let mut xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().distance, -1.0);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 1.0);
}

#[test]
fn ch5_test7_ray_intersects_when_sphere_behind() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere: Object = Sphere::builder().build().into();
    let mut xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().distance, -6.0);
    assert_eq!(xs.hit_unchecked().unwrap().distance, -4.0);
}

#[test]
fn ch5_test10_intersect_sets_object_on_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s: Object = Sphere::builder().build().into();
    let mut xs = s.intersect(&ray);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().object, &s);
    assert_eq!(xs.hit_unchecked().unwrap().object, &s);
}

#[test]
fn ch5_test13_sphere_has_default_and_updatable_transformation() {
    let mut s: Object = Sphere::builder().build().into();
    assert_eq!(
        s.shape().transformation,
        crate::domain::matrix::IDENTITY.clone()
    );

    let m = Matrix::new_translation(2.0, 3.0, 4.0);
    s.shape_mut().transformation = m.clone();
    assert_ne!(
        s.shape().transformation,
        crate::domain::matrix::IDENTITY.clone()
    );
    assert_eq!(m, s.shape().transformation);
}

#[test]
fn ch5_test14_intersecting_scaled_translated_sphere_with_ray() {
    // scaled
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let m = Matrix::new_scaling(2.0, 2.0, 2.0);
    let s: Object = Sphere::builder().transformation(m).build().into();
    let mut xs = s.intersect(&ray);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 3.0);
    assert_eq!(xs.hit_unchecked().unwrap().distance, 7.0);

    // translated
    let m = Matrix::new_translation(5.0, 0.0, 0.0);
    let s: Object = Sphere::builder().transformation(m).build().into();
    let xs = s.intersect(&ray);

    assert!(xs.is_empty());
}

#[test]
fn ch6_test1_normal_on_a_sphere() {
    // normal on an x-axis
    let s: Object = Sphere::builder().build().into();
    let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
    let exp_n = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a y-axis
    let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
    let exp_n = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a z-axis
    let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
    let exp_n = Vector::new(0.0, 0.0, 1.0);
    assert_eq!(n, exp_n);

    // normal on a nonaxial point
    let n = s.normal_at(&Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let exp_n = Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    assert_eq!(n, exp_n);
}

#[test]
fn ch6_test2_normal_is_normalized() {
    let s: Object = Sphere::builder().build().into();
    let n = s.normal_at(&Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let normalized = n.normalize();

    assert_eq!(n, normalized);
}

#[test]
fn ch6_test3_computing_normal_of_modified_sphere() {
    // translated sphere
    let t = Matrix::new_translation(0.0, 1.0, 0.0);
    let s: Object = Sphere::builder().transformation(t).build().into();
    let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
    let exp_n = Vector::new(0.0, 0.70711, -0.70711);

    assert_eq!(n, exp_n);

    let t = &Matrix::new_scaling(1.0, 0.5, 1.0) * &Matrix::new_rotation_z(PI / 0.5);
    let s: Object = Sphere::builder().transformation(t).build().into();
    let n = s.normal_at(&Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    let exp_n = Vector::new(0.0, 0.97014, -0.24254);

    assert_eq!(n, exp_n);
}

#[test]
fn ch6_test8_sphere_has_material() {
    // default material
    let s: Object = Sphere::builder().build().into();
    let m_exp = Material::default();

    assert_eq!(s.shape().material, m_exp);

    // can be assigned material
    let c = Color::new(0.5, 0.5, 0.5);
    let m = Material::new().color(c).ambient(1.0).build();
    let s: Object = Sphere::builder().material(m).build().into();
    assert_eq!(s.shape().material.ambient, 1.0);
}

#[test]
fn ch11_test9_produce_sphere_with_glassy_material() {
    let s = Sphere::builder()
        .material(
            Material::new()
                .transparency(1.0)
                .substance(Substance::GLASS)
                .build(),
        )
        .build();
    assert_eq!(
        s.shape.transformation,
        crate::domain::matrix::IDENTITY.clone()
    );
    assert_eq!(s.shape.material.transparency, 1.0);
    assert_eq!(s.shape.material.substance.refractive_index(), 1.52);
}
