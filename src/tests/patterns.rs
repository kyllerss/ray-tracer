use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::pattern::Pattern;
use crate::domain::{Point, Vector};

#[test]
fn ch10_test1_creating_stripe_pattern() {
    let Pattern::STRIPED { a, b } = Pattern::new_striped(Color::WHITE, Color::BLACK);
    assert_eq!(a, Color::WHITE);
    assert_eq!(b, Color::BLACK);
}

#[test]
fn ch10_test2_stripe_pattern_constant_in_y_z_and_alternating_in_x() {
    let pattern = Pattern::new_striped(Color::WHITE, Color::BLACK);

    // y
    assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(0.0, 2.0, 0.0)), &Color::WHITE);

    // z
    assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 2.0)), &Color::WHITE);

    // x
    assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), &Color::WHITE);
    assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), &Color::BLACK);
    assert_eq!(pattern.color_at(&Point::new(-0.1, 0.0, 0.0)), &Color::BLACK);
    assert_eq!(pattern.color_at(&Point::new(-1.0, 0.0, 0.0)), &Color::BLACK);
    assert_eq!(pattern.color_at(&Point::new(-1.1, 0.0, 0.0)), &Color::WHITE);
}

#[test]
fn ch10_test3_lighting_with_pattern_applied() {
    let m = Material::new_full(
        Color::WHITE,
        1.0,
        0.0,
        0.0,
        Material::DEFAULT_SHININESS,
        Some(Pattern::new_striped(Color::WHITE, Color::BLACK)),
    );
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal_v = Vector::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
    let c1 = Light::lighting(
        &m,
        &light,
        &Point::new(0.9, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );
    let c2 = Light::lighting(
        &m,
        &light,
        &Point::new(1.1, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::BLACK);
}
