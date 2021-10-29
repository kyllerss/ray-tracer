use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersection, Intersections};
use crate::domain::light::Light;
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Plane, Sphere};
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
    let m1 = Material::new().color(mc).diffuse(0.7).specular(0.2).build();
    let s1: Object = Sphere::new().material(m1).build().into();

    let t2 = Matrix::new_scaling(0.5, 0.5, 0.5);
    let s2: Object = Sphere::new().transformation(t2).build().into();

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
    let m1 = Material::new().color(mc).diffuse(0.7).specular(0.2).build();
    let s1_exp: Object = Sphere::new().material(m1).build().into();
    assert!(w.objects.contains(&s1_exp));

    let t2 = Matrix::new_scaling(0.5, 0.5, 0.5);
    let s2_exp: Object = Sphere::new().transformation(t2).build().into();
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
    let shape = w.objects.first().unwrap();
    let i = Intersection::new(4.0, shape);
    let comps = Computations::prepare_computations(&i, &r);
    let c: Color = w.shade_hit(&comps, 1);
    let c_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(c, c_exp);

    // inside
    w.light_source = Some(Light::new(Point::new(0.0, 0.25, 0.0), Color::WHITE));
    let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects.get(1).unwrap();
    let i = Intersection::new(0.5, shape);
    let comps = Computations::prepare_computations(&i, &r);
    let c = w.shade_hit(&comps, 1);
    let c_exp = Color::new(0.90498, 0.90498, 0.90498);
    assert_eq!(c, c_exp);
}

#[test]
fn ch7_test7_color_when_ray_misses() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
    let c = w.color_at(&r, 1);
    let c_exp = Color::BLACK;
    assert_eq!(c, c_exp);
}

#[test]
fn ch7_test8_color_when_ray_hits() {
    let w = build_test_world();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let c = w.color_at(&r, 1);
    let c_exp = Color::new(0.38066, 0.47583, 0.2855);
    assert_eq!(c, c_exp);
}

#[test]
fn ch7_test9_color_intersection_behind_ray() {
    let mut w = build_test_world();
    w.objects[0].shape_mut().material.ambient = 1.0;
    w.objects[1].shape_mut().material.ambient = 1.0;
    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    let c = w.color_at(&r, 1);
    let c_exp = w.objects[1].shape().material.color;
    assert_eq!(c, c_exp);
}

#[test]
fn ch8_test2_no_shadow_when_nothing_collinear_with_point_and_light() {
    let w = build_test_world();
    let p = Point::new(0.0, 10.0, 0.0);
    let r = w.is_shadowed(&p);
    assert_eq!(r, false);
}

#[test]
fn ch8_test3_shadow_when_object_between_point_and_light() {
    let w = build_test_world();
    let p = Point::new(10.0, -10.0, 10.0);
    let r = w.is_shadowed(&p);
    assert_eq!(r, true);
}

#[test]
fn ch8_test4_no_shadow_when_object_behind_light() {
    let w = build_test_world();
    let p = Point::new(-20.0, 20.0, -20.0);
    let r = w.is_shadowed(&p);
    assert_eq!(r, false);
}

#[test]
fn ch8_test5_no_shadow_when_object_behind_point() {
    let w = build_test_world();
    let p = Point::new(-2.0, 2.0, -2.0);
    let r = w.is_shadowed(&p);
    assert_eq!(r, false);
}

#[test]
fn ch8_test6_shade_hit_is_given_intersection_in_shadow() {
    let mut w = World::new();
    w.light_source = Some(Light::new(Point::new(0.0, 0.0, -10.0), Color::WHITE));
    let s1: Object = Sphere::new().build().into();
    w.objects.push(s1);
    let s2: Object = Sphere::new()
        .transformation(Matrix::new_translation(0.0, 0.0, 10.0))
        .build()
        .into();
    w.objects.push(s2.clone());
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, &s2);
    let comps = Computations::prepare_computations(&i, &r);
    let r = w.shade_hit(&comps, 1);
    let r_exp = Color::new(0.1, 0.1, 0.1);
    assert_eq!(r, r_exp);
}

#[test]
fn ch11_test3_reflected_color_for_nonreflective_material() {
    let mut w = build_test_world();
    let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));

    // https://stackoverflow.com/questions/69771391/mutable-borrow-followed-by-immutable-borrow-cannot-borrow-as-immutable-because?noredirect=1#comment123330186_69771391
    let shape = {
        let shape = w.objects.get_mut(1).unwrap();
        shape.shape_mut().material.ambient = 1.0;

        let immutable = w.objects.get(1).unwrap();
        immutable
    };
    let i = Intersection::new(1.0, shape);
    let comps = Computations::prepare_computations(&i, &r);
    let color = w.reflected_color(&comps, 1);
    assert_eq!(color, Color::BLACK);
}

#[test]
fn ch11_test4_reflected_color_for_reflected_material() {
    let mut w = build_test_world();
    let shape: Object = Plane::new()
        .material(Material::new().reflective(0.5).build())
        .transformation(Matrix::new_translation(0.0, -1.0, 0.0))
        .build()
        .into();
    w.add_object(shape.clone());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Computations::prepare_computations(&i, &r);
    let color = w.reflected_color(&comps, 1);
    assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
}

#[test]
fn ch11_test5_shade_hit_with_reflective_material() {
    let mut w = build_test_world();
    let shape: Object = Plane::new()
        .material(Material::new().reflective(0.5).build())
        .transformation(Matrix::new_translation(0.0, -1.0, 0.0))
        .build()
        .into();
    w.add_object(shape.clone());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Computations::prepare_computations(&i, &r);
    let color = w.shade_hit(&comps, 1);
    assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
}

#[test]
fn ch11_test6_color_at_with_mutually_reflective_surfaces() {
    let mut w = World::new();
    w.light_source = Option::Some(Light::new(Point::ORIGIN, Color::WHITE));

    let lower = Plane::new()
        .material(Material::new().reflective(1.0).build())
        .transformation(Matrix::new_translation(0.0, -1.0, 0.0))
        .build()
        .into();
    w.add_object(lower);

    let upper = Plane::new()
        .material(Material::new().reflective(1.0).build())
        .transformation(Matrix::new_translation(0.0, 1.0, 0.0))
        .build()
        .into();
    w.add_object(upper);

    let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 1.0, 0.0));
    let _ = w.color_at(&r, 1); // should terminate successfully
    assert!(true);
}

#[test]
fn ch11_test7_reflected_color_for_reflected_material() {
    let mut w = build_test_world();
    let shape: Object = Plane::new()
        .material(Material::new().reflective(0.5).build())
        .transformation(Matrix::new_translation(0.0, -1.0, 0.0))
        .build()
        .into();
    w.add_object(shape.clone());
    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Computations::prepare_computations(&i, &r);
    let color = w.reflected_color(&comps, 0);
    assert_eq!(color, Color::BLACK);
}
