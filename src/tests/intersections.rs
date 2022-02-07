use crate::domain::intersection::{Computations, Intersection, Intersections};
use crate::domain::material::{Material, Substance};
use crate::domain::matrix::Matrix;
use crate::domain::object::{Object, Plane, Sphere, Triangle};
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch5_test8_intersection_object_encapsulates_t_and_obj() {
    let distance = 3.5;
    let sphere: Object = Sphere::builder().build().into();
    let intersection = Intersection::new(distance, &sphere);
    assert_eq!(intersection.distance, distance);
    assert_eq!(intersection.object, &sphere);
}

#[test]
fn ch5_test9_aggregating_intersections() {
    let s: Object = Sphere::builder().build().into();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs.hit().unwrap().distance, 1.0);
    assert_eq!(xs.hit().unwrap().distance, 2.0);
}

#[test]
fn ch5_test11_hit_tests() {
    // all intersections positive
    let s: Object = Sphere::builder().build().into();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let mut xs = Intersections::new();
    xs.push(i2);
    xs.push(i1);

    let hit = xs.hit();
    assert!(hit.is_some());
    let int = hit.unwrap();
    assert_eq!(int.distance, 1.0);
    assert_eq!(int.object, &s);

    // some intersections negative
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);
    let mut xs = Intersections::new();
    xs.push(i2);
    xs.push(i1);

    let hit = xs.hit();
    assert!(hit.is_some());
    let int = hit.unwrap();
    assert_eq!(int.distance, 1.0);
    assert_eq!(int.object, &s);

    // all intersections have negative
    let i1 = Intersection::new(-2.0, &s);
    let i2 = Intersection::new(-1.0, &s);
    let mut xs = Intersections::new();
    xs.push(i2);
    xs.push(i1);

    let hit = xs.hit();
    assert!(hit.is_none());

    // always lowest non-negative intersection
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(7.0, &s);
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);
    xs.push(i3);
    xs.push(i4);

    let hit = xs.hit();
    assert!(hit.is_some());
    let int = hit.unwrap();
    assert_eq!(int.distance, 2.0);
    assert_eq!(int.object, &s);

    // EDGE CASES (comparison w/ f64 NAN, INFINIT, etc...
    let i1 = Intersection::new(f64::INFINITY, &s);
    let i2 = Intersection::new(f64::NEG_INFINITY, &s);
    let i3 = Intersection::new(f64::NAN, &s);
    let i_good = Intersection::new(0.0, &s);
    let i5 = Intersection::new(f64::INFINITY, &s);
    let i6 = Intersection::new(f64::NEG_INFINITY, &s);
    let i7 = Intersection::new(f64::NAN, &s);
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);
    xs.push(i3);
    xs.push(i_good);
    xs.push(i5);
    xs.push(i6);
    xs.push(i7);

    let hit = xs.hit();
    assert!(hit.is_some());
    let int = hit.unwrap();
    assert_eq!(int.distance, 0.0);
    assert_eq!(int.object, &s);
}

#[test]
fn ch7_test4_precomputing_state_of_intersection() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: Object = Sphere::builder().build().into();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r, Option::None);
    assert_eq!(comps.distance, i.distance);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
    assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn ch7_test5_prepare_computations_when_hit_outside_and_inside() {
    // outside hit
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: Object = Sphere::builder().build().into();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r, Option::None);
    assert_eq!(comps.inside, false);

    // inside hit
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let i = Intersection::new(1.0, &shape);
    let comps = Computations::prepare_computations(&i, &r, Option::None);
    assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
    assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn ch8_test7_hit_should_offset_point() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: Object = Sphere::builder()
        .transformation(Matrix::new_translation(0.0, 0.0, 1.0))
        .build()
        .into();
    let i = Intersection::new(5.0, &shape);
    let comps = Computations::prepare_computations(&i, &r, Option::None);
    assert!(comps.over_point.z() < -crate::domain::EPSILON / 2.0);
    assert!(comps.point.z() > comps.over_point.z());
}

#[test]
fn ch11_test2_precomputing_reflection_vector() {
    let shape: Object = Plane::builder().build().into();
    let r = Ray::new(
        Point::new(0.0, 1.0, -1.0),
        Vector::new(0.0, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64),
    );
    let i = Intersection::new(2_f64.sqrt(), &shape);
    let comps = Computations::prepare_computations(&i, &r, Option::None);
    assert_eq!(
        comps.reflect_v,
        Vector::new(0.0, 2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64)
    );
}

#[test]
fn ch11_test10_finding_n1_and_n2_at_various_intersections() {
    let a = Sphere::builder()
        .material(
            Material::new()
                .transparency(1.0)
                .refractive_index_override(1.5)
                .build(),
        )
        .transformation(Matrix::new_scaling(2.0, 2.0, 2.0))
        .build()
        .into();
    let b = Sphere::builder()
        .material(
            Material::new()
                .transparency(1.0)
                .refractive_index_override(2.0)
                .build(),
        )
        .transformation(Matrix::new_translation(0.0, 0.0, -0.25))
        .build()
        .into();
    let c = Sphere::builder()
        .material(
            Material::new()
                .transparency(1.0)
                .refractive_index_override(2.5)
                .build(),
        )
        .transformation(Matrix::new_translation(0.0, 0.0, 0.25))
        .build()
        .into();
    let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));

    let int1 = Intersection::new(2.0, &a);
    let int2 = Intersection::new(2.75, &b);
    let int3 = Intersection::new(3.25, &c);
    let int4 = Intersection::new(4.75, &b);
    let int5 = Intersection::new(5.25, &c);
    let int6 = Intersection::new(6.0, &a);
    let ints_container = vec![int1, int2, int3, int4, int5, int6];

    let mut ints = Intersections::new();
    ints.push(int1);
    ints.push(int2);
    ints.push(int3);
    ints.push(int4);
    ints.push(int5);
    ints.push(int6);

    let runs: [(f64, f64); 6] = [
        (1.0, 1.5),
        (1.5, 2.0),
        (2.0, 2.5),
        (2.5, 2.5),
        (2.5, 1.5),
        (1.5, 1.0),
    ];

    let run = 0;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);
    let run = 1;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);
    let run = 2;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);
    let run = 3;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);
    let run = 4;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);
    let run = 5;
    let comps = Computations::prepare_computations(&ints_container[run], &r, Option::Some(&ints));
    assert_eq!(comps.n1, runs[run].0);
    assert_eq!(comps.n2, runs[run].1);

    // runs.iter().enumerate().for_each(|(i, (i1, i2))| {
    //     let comps = Computations::prepare_computations(&ints_container[i], &r, Option::Some(&ints));
    //     assert_eq!(comps.n1, *i1);
    //     assert_eq!(comps.n2, *i2);
    // });
}

#[test]
fn ch11_test11_under_point_is_offset_below_the_surface() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: Object = Sphere::builder()
        .transformation(Matrix::new_translation(0.0, 0.0, 1.0))
        .material(Material::new().substance(Substance::GLASS).build())
        .build()
        .into();
    let i = Intersection::new(5.0, &shape);
    let xs = {
        let mut xs = Intersections::new();
        xs.push(i.clone());
        xs
    };
    let comps = Computations::prepare_computations(&i, &r, Option::Some(&xs));
    assert!(comps.under_point.z() > crate::domain::EPSILON / 2.0);
    assert!(comps.point.z() < comps.under_point.z());
}

#[test]
fn ch11_test17_schlick_approximation_under_total_internal_reflection() {
    let shape = Sphere::builder()
        .material(
            Material::new()
                .refractive_index_override(1.5)
                .transparency(1.0)
                .build(),
        )
        .build()
        .into();
    let r = Ray::new(
        Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let int = Intersection::new(2_f64.sqrt() / 2.0, &shape);
    let xs = {
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-(2_f64.sqrt() / 2.0), &shape));
        xs.push(int.clone());
        xs
    };
    let comps = Computations::prepare_computations(&int, &r, Option::Some(&xs));
    let reflectance = comps.schlick();
    assert_eq!(reflectance, 1.0);
}

#[test]
fn ch11_test18_schlick_approximation_with_perpendicular_viewing_angle() {
    let shape = Sphere::builder()
        .material(
            Material::new()
                .refractive_index_override(1.5)
                .transparency(1.0)
                .build(),
        )
        .build()
        .into();
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let int = Intersection::new(1.0, &shape);
    let xs = {
        let mut xs = Intersections::new();
        xs.push(Intersection::new(-1.0, &shape));
        xs.push(int.clone());
        xs
    };
    let comps = Computations::prepare_computations(&int, &r, Option::Some(&xs));
    let reflectance = comps.schlick();
    assert!(crate::domain::epsilon_eq(reflectance, 0.04));
}

#[test]
fn ch11_test19_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
    let shape = Sphere::builder()
        .material(
            Material::new()
                .refractive_index_override(1.5)
                .transparency(1.0)
                .build(),
        )
        .build()
        .into();
    let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
    let int = Intersection::new(1.8589, &shape);
    let xs = {
        let mut xs = Intersections::new();
        xs.push(int.clone());
        xs
    };
    let comps = Computations::prepare_computations(&int, &r, Option::Some(&xs));
    let reflectance = comps.schlick();
    assert!(crate::domain::epsilon_eq(reflectance, 0.48873));
}

#[test]
fn ch15_test14_intersection_can_encapsulate_u_and_v() {
    let s = Triangle::builder(
        Point::new(0.0, 1.0, 0.0),
        Point::new(-1.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    )
    .build()
    .into();
    let i = Intersection::new_with_uv(3.5, &s, 0.2, 0.4);
    assert_eq!(i.u, Option::Some(0.2));
    assert_eq!(i.v, Option::Some(0.4));
}
