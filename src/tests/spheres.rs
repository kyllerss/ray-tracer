use crate::domain::color::Color;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

#[test]
fn ch5_test3_ray_intersects_sphere_at_two_points() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].distance, 4.0);
    assert_eq!(xs[1].distance, 6.0);
}

#[test]
fn ch5_test4_ray_intersects_sphere_at_tangent() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].distance, 5.0);
    assert_eq!(xs[1].distance, 5.0);
}

#[test]
fn ch5_test5_ray_misses_intersection_on_sphere() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert!(xs.is_empty());
}

#[test]
fn ch5_test6_ray_intersects_sphere_when_ray_at_origin() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].distance, -1.0);
    assert_eq!(xs[1].distance, 1.0);
}

#[test]
fn ch5_test7_ray_intersects_when_sphere_behind() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].distance, -6.0);
    assert_eq!(xs[1].distance, -4.0);
}

#[test]
fn ch5_test10_intersect_sets_object_on_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    let xs = s.intersect(&ray);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, &s);
    assert_eq!(xs[1].object, &s);
}

#[test]
fn ch5_test13_sphere_has_default_and_updatable_transformation() {
    let mut s = Sphere::new_unit();
    assert_eq!(s.transformation, crate::domain::matrix::IDENTITY.clone());

    let m = Matrix::new_translation(2.0, 3.0, 4.0);
    s.transformation = m.clone();
    assert_ne!(s.transformation, crate::domain::matrix::IDENTITY.clone());
    assert_eq!(m, s.transformation);
}

#[test]
fn ch5_test14_intersecting_scaled_translated_sphere_with_ray() {
    // scaled
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let m = Matrix::new_scaling(2.0, 2.0, 2.0);
    let s = Sphere::new(m);
    let xs = s.intersect(&ray);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].distance, 3.0);
    assert_eq!(xs[1].distance, 7.0);

    // translated
    let m = Matrix::new_translation(5.0, 0.0, 0.0);
    let s = Sphere::new(m);
    let xs = s.intersect(&ray);

    assert!(xs.is_empty());
}

#[test]
fn ch6_test1_normal_on_a_sphere() {
    // normal on an x-axis
    let s = Sphere::new_unit();
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
    let s = Sphere::new_unit();
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
    let s = Sphere::new(t);
    let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
    let exp_n = Vector::new(0.0, 0.70711, -0.70711);

    assert_eq!(n, exp_n);

    let t = &Matrix::new_scaling(1.0, 0.5, 1.0) * &Matrix::new_rotation_z(PI / 0.5);
    let s = Sphere::new(t);
    let n = s.normal_at(&Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    let exp_n = Vector::new(0.0, 0.97014, -0.24254);

    assert_eq!(n, exp_n);
}

#[test]
fn ch6_test8_sphere_has_material() {
    // default material
    let s = Sphere::new_unit();
    let m_exp = Material::new();

    assert_eq!(s.material, m_exp);

    // can be assigned material
    let c = Color::new(0.5, 0.5, 0.5);
    let m = Material::new_full(
        c,
        1.0,
        Material::DEFAULT_DIFFUSE,
        Material::DEFAULT_SPECULAR,
        Material::DEFAULT_SHININESS,
    );
    let s = Sphere::new_material(m);
    assert_eq!(s.material.ambient, 1.0);
}
