use crate::domain::intersection::{Computations, Intersection, Intersections};
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch5_test8_intersection_object_encapsulates_t_and_obj() {
    let distance = 3.5;
    let sphere = Sphere::new_unit();
    let intersection = Intersection::new(distance, &sphere);
    assert_eq!(intersection.distance, distance);
    assert_eq!(intersection.object, &sphere);
}

#[test]
fn ch5_test9_aggregating_intersections() {
    let s = Sphere::new_unit();
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
    let s = Sphere::new_unit();
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
    let shape = Sphere::new_unit();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
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
    let shape = Sphere::new_unit();
    let i = Intersection::new(4.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
    assert_eq!(comps.inside, false);

    // inside hit
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let i = Intersection::new(1.0, &shape);
    let comps = Computations::prepare_computations(&i, &r);
    assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
    assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
}