use crate::domain::color::Color;
use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::world::{Computations, World};
use crate::domain::{Point, Vector};

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
    let mut xs: Intersections = w.intersect(r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs.hit().unwrap().distance, 4.0);
    assert_eq!(xs.hit().unwrap().distance, 4.5);
    assert_eq!(xs.hit().unwrap().distance, 5.5);
    assert_eq!(xs.hit().unwrap().distance, 6.0);
}

#[test]
fn test_4_precomputing_state_of_intersection() {
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
