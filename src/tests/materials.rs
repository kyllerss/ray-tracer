use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::object::{Object, Sphere};
use crate::domain::{Point, Vector};

#[test]
fn ch0_force_unused_methods_to_be_used() {
    let _m = Material::new().shininess(1.0).build();
}

#[test]
fn ch6_test7_default_material() {
    let m = Material::default();
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

fn generate_test_harness_lighting(
    eyev_y: f64,
    eyev_z: f64,
    lightpoint_y: f64,
    lightpoint_z: f64,
) -> Color {
    let m = Material::default();
    let object: Object = Sphere::new().build().into();
    let position = Point::new(0.0, 0.0, 0.0);
    let eye_v = Vector::new(0.0, eyev_y, eyev_z);
    let normal_v = Vector::new(0.0, 0.0, -1.0);
    let light = Light::new(
        Point::new(0.0, lightpoint_y, lightpoint_z),
        Color::new(1.0, 1.0, 1.0),
    );
    Light::lighting(&m, &object, &light, &position, &eye_v, &normal_v, false)
}

#[test]
fn ch6_test9_lighting_eye_between_light_and_surface() {
    let result = generate_test_harness_lighting(0.0, -1.0, 0.0, -10.0);
    let exp = Color::new(1.9, 1.9, 1.9);
    assert_eq!(result, exp);
}

#[test]
fn ch6_test10_lighting_eye_between_light_and_surface_offset_by_45() {
    let result = generate_test_harness_lighting(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0, -10.0);
    let exp = Color::new(1.0, 1.0, 1.0);
    assert_eq!(result, exp);
}

#[test]
fn ch6_test11_lighting_eye_opposite_surface_light_offset_45() {
    let result = generate_test_harness_lighting(0.0, -1.0, 10.0, -10.0);
    let exp = Color::new(0.7364, 0.7364, 0.7364);
    assert_eq!(result, exp);
}

#[test]
fn ch6_test12_lighting_with_eye_in_path_of_reflection_vector() {
    let result =
        generate_test_harness_lighting(-2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0, 10.0, -10.0);
    let exp = Color::new(1.6364, 1.6364, 1.6364);
    assert_eq!(result, exp);
}

#[test]
fn ch6_test13_lighting_with_light_behind_surface() {
    let result = generate_test_harness_lighting(0.0, -1.0, 0.0, 10.0);
    let exp = Color::new(0.1, 0.1, 0.1);
    assert_eq!(result, exp);
}

#[test]
fn ch8_test1_lighting_with_surface_in_shadow() {
    let m = Material::default();
    let object: Object = Sphere::new().build().into();
    let position = Point::ORIGIN;
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal_v = Vector::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
    let in_shadow = true;
    let result = Light::lighting(&m, &object, &light, &position, &eye_v, &normal_v, in_shadow);
    let result_exp = Color::new(0.1, 0.1, 0.1);
    assert_eq!(result, result_exp);
}
