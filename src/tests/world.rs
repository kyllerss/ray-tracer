use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersection, Intersections};
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::world::World;
use crate::domain::{Point, Vector};

#[test]
fn ch7_test1_world_can_be_constructed() {
    let w = World::new();
    assert!(w.objects.is_empty());
    assert_eq!(w.light_source, Option::None);
}

pub fn build_test_world() -> World {
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
fn ch7_test2_validate_default_world() {
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
fn ch7_test3_intersect_world_with_ray() {
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
fn ch7_test6_shading_an_intersection_inside_and_outside() {
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
fn ch7_test7_color_when_ray_misses() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
    let c = w.color_at(&r);
    let c_exp = Color::BLACK;
    assert_eq!(c, c_exp);
}

#[test]
fn ch7_test8_color_when_ray_hits() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(&r);
    let c_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(c, c_exp);
}

#[test]
fn ch7_test9_color_intersection_behind_ray() {
    let mut w = build_test_world();
    w.objects[0].material.ambient = 1.0;
    w.objects[1].material.ambient = 1.0;
    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    let c = w.color_at(&r);
    let c_exp = w.objects[1].material.color;
    assert_eq!(c, c_exp);
}