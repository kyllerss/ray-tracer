use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::pattern::Pattern;
use crate::domain::{Point, Vector};

#[test]
fn ch10_test1_creating_stripe_pattern() {
    if let Pattern::STRIPED { a, b, .. } = Pattern::new_striped(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    ) {
        assert_eq!(a, Color::WHITE);
        assert_eq!(b, Color::BLACK);
    } else {
        panic!("Unexpected result!");
    }
}

#[test]
fn ch10_test2_stripe_pattern_constant_in_y_z_and_alternating_in_x() {
    let pattern = Pattern::new_striped(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    );

    // y
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 0.0, 0.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 1.0, 0.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 2.0, 0.0)
        ),
        Color::WHITE
    );

    // z
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 0.0, 0.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 0.0, 1.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 0.0, 2.0)
        ),
        Color::WHITE
    );

    // x
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.0, 0.0, 0.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(0.9, 0.0, 0.0)
        ),
        Color::WHITE
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(1.0, 0.0, 0.0)
        ),
        Color::BLACK
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(-0.1, 0.0, 0.0)
        ),
        Color::BLACK
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(-1.0, 0.0, 0.0)
        ),
        Color::BLACK
    );
    assert_eq!(
        pattern.color_at(
            &Sphere::builder().build().into(),
            &Point::new(-1.1, 0.0, 0.0)
        ),
        Color::WHITE
    );
}

#[test]
fn ch10_test3_lighting_with_pattern_applied() {
    let m = Material::new()
        .ambient(1.0)
        .diffuse(0.0)
        .specular(0.0)
        .pattern(Pattern::new_striped(
            Color::WHITE,
            Color::BLACK,
            crate::domain::matrix::IDENTITY.clone(),
        ))
        .build();
    let object = &Sphere::builder().build().into();
    let eye_v = Vector::new(0.0, 0.0, -1.0);
    let normal_v = Vector::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
    let c1 = Light::lighting(
        &m,
        &object,
        &light,
        &Point::new(0.9, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );
    let c2 = Light::lighting(
        &m,
        &object,
        &light,
        &Point::new(1.1, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::BLACK);
}

#[test]
fn ch10_test4_stripes_with_object_and_pattern_transformations() {
    // stripes with an object transformation
    let object = Sphere::builder()
        .transformation(Matrix::new_scaling(2.0, 2.0, 2.0))
        .build()
        .into();
    let pattern = Pattern::new_striped(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    );
    let point = Point::new(1.5, 0.0, 0.0);
    let c = pattern.color_at(&object, &point);
    assert_eq!(c, Color::WHITE);

    // stripes with a pattern transformation
    let object = Sphere::builder().build().into();
    let pattern = Pattern::new_striped(
        Color::WHITE,
        Color::BLACK,
        Matrix::new_scaling(2.0, 2.0, 2.0),
    );
    let point = Point::new(1.5, 0.0, 0.0);
    let c = pattern.color_at(&object, &point);
    assert_eq!(c, Color::WHITE);

    // stripes with both object and pattern transformations
    let object = Sphere::builder()
        .transformation(Matrix::new_scaling(2.0, 2.0, 2.0))
        .build()
        .into();
    let pattern = Pattern::new_striped(
        Color::WHITE,
        Color::BLACK,
        Matrix::new_translation(0.5, 0.0, 0.0),
    );
    let point = Point::new(2.5, 0.0, 0.0);
    let c = pattern.color_at(&object, &point);
    assert_eq!(c, Color::WHITE);
}

// Skipping ch10_test5 as superfluous.

#[test]
fn ch10_test6_gradient_linearly_interpolates_between_colors() {
    let pattern = Pattern::new_gradient(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    );
    let object = Sphere::builder().build().into();
    let c1 = pattern.color_at(&object, &Point::new(0.0, 0.0, 0.0));
    let c2 = pattern.color_at(&object, &Point::new(0.25, 0.0, 0.0));
    let c3 = pattern.color_at(&object, &Point::new(0.5, 0.0, 0.0));
    let c4 = pattern.color_at(&object, &Point::new(0.75, 0.0, 0.0));

    assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
    assert_eq!(c2, Color::new(0.75, 0.75, 0.75));
    assert_eq!(c3, Color::new(0.5, 0.5, 0.5));
    assert_eq!(c4, Color::new(0.25, 0.25, 0.25));
}

#[test]
fn ch10_test7_ring_pattern() {
    let pattern = Pattern::new_ringed(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    );
    let obj = Sphere::builder().build().into();
    let c1 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 0.0));
    let c2 = pattern.color_at(&obj, &Point::new(1.0, 0.0, 0.0));
    let c3 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 1.0));
    let c4 = pattern.color_at(&obj, &Point::new(0.708, 0.0, 0.708));

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::BLACK);
    assert_eq!(c3, Color::BLACK);
    assert_eq!(c4, Color::BLACK);
}

#[test]
fn ch10_test8_checkered_pattern() {
    let pattern = Pattern::new_checkered(
        Color::WHITE,
        Color::BLACK,
        crate::domain::matrix::IDENTITY.clone(),
    );
    let obj = Sphere::builder().build().into();

    // repeats in x
    let c1 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 0.0));
    let c2 = pattern.color_at(&obj, &Point::new(0.99, 0.0, 0.0));
    let c3 = pattern.color_at(&obj, &Point::new(1.01, 0.0, 0.0));

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::WHITE);
    assert_eq!(c3, Color::BLACK);

    // repeats in y
    let c1 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 0.0));
    let c2 = pattern.color_at(&obj, &Point::new(0.0, 0.99, 0.0));
    let c3 = pattern.color_at(&obj, &Point::new(0.0, 1.01, 0.0));

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::WHITE);
    assert_eq!(c3, Color::BLACK);

    // repeats in z
    let c1 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 0.0));
    let c2 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 0.99));
    let c3 = pattern.color_at(&obj, &Point::new(0.0, 0.0, 1.01));

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::WHITE);
    assert_eq!(c3, Color::BLACK);
}
