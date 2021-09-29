use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

#[test]
fn test1_normal_on_a_sphere() {
    // normal on an x-axis
    let s = Sphere::new_unit();
    let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
    let exp_n = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a y-axis
    let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
    let exp_n = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a z-axis
    let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
    let exp_n = Vector::new(0.0, 0.0, 1.0);
    assert_eq!(n, exp_n);

    // normal on a nonaxial point
    let n = s.normal_at(Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let exp_n = Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    assert_eq!(n, exp_n);
}

#[test]
fn test2_normal_is_normalized() {
    let s = Sphere::new_unit();
    let n = s.normal_at(Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let normalized = n.normalize();

    assert_eq!(n, normalized);
}

#[test]
fn test3_computing_normal_of_modified_sphere() {
    // translated sphere
    let t = Matrix::new_translation(0.0, 1.0, 0.0);
    let s = Sphere::new(t);
    let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
    let exp_n = Vector::new(0.0, 0.70711, -0.70711);

    assert_eq!(n, exp_n);

    let t = &Matrix::new_scaling(1.0, 0.5, 1.0) * &Matrix::new_rotation_z(PI / 0.5);
    let s = Sphere::new(t);
    let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    let exp_n = Vector::new(0.0, 0.97014, -0.24254);

    assert_eq!(n, exp_n);
}

#[test]
fn test4_reflecting_vector_at_45_deg() {
    let v = Vector::new(1.0, -1.0, 0.0);
    let n = Vector::new(0.0, 1.0, 0.0);
    let r = v.reflect(&n);
    let exp_r = Vector::new(1.0, 1.0, 0.0);

    assert_eq!(r, exp_r);
}

#[test]
fn test5_reflecting_vector_at_slanted_surface() {
    let v = Vector::new(0.0, -1.0, 0.0);
    let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
    let r = v.reflect(&n);
    let exp_r = Vector::new(1.0, 0.0, 0.0);

    assert_eq!(r, exp_r);
}

#[test]
fn test6_point_light_has_position_and_intensity() {
    let i_exp = Color::new(1.0, 1.0, 1.0);
    let p_exp = Point::new(0.0, 0.0, 0.0);
    let light = Light::new(p_exp.clone(), i_exp.clone());
    assert_eq!(light.position, p_exp);
    assert_eq!(light.intensity, i_exp);
}

#[test]
fn test7_default_material() {
    let m = Material::new();
    let color_exp = Color::new(1.0, 1.0, 1.0);
    let ambient_exp = 0.1;
    let diffuse_exp = 0.9;
    let specular_exp = 0.9;
    let shininess_exp = 200.0;
    assert_eq!(m.color, color_exp);
    assert_eq!(m.ambient, ambient_exp);
    assert_eq!(m.diffuse, diffuse_exp);
    assert_eq!(m.specular, specular_exp);
    assert_eq!(m.shininess, shininess_exp);
}

#[test]
fn test8_sphere_has_material() {
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
