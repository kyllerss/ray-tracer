use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::world::World;
use crate::domain::Point;

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
