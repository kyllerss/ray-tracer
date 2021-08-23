use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn test1_creating_querying_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn test2_computing_point_from_distance() {
    let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

    // distance 0
    let p = r.position(0.0);
    let exp = Point::new(2.0, 3.0, 4.0);
    assert_eq!(p, exp);

    // distance 1
    let p = r.position(1.0);
    let exp = Point::new(3.0, 3.0, 4.0);
    assert_eq!(p, exp);

    // distance -1
    let p = r.position(-1.0);
    let exp = Point::new(1.0, 3.0, 4.0);
    assert_eq!(p, exp);

    // distance 2.5
    let p = r.position(2.5);
    let exp = Point::new(4.5, 3.0, 4.0);
    assert_eq!(p, exp);
}

#[test]
fn test3_ray_intersects_sphere_at_two_points() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}

#[test]
fn test4_ray_intersects_sphere_at_tangent() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}

#[test]
fn test5_ray_misses_intersection_on_sphere() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert!(xs.is_empty());
}

#[test]
fn test6_ray_intersects_sphere_when_ray_at_origin() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}

#[test]
fn test7_ray_intersects_when_sphere_behind() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::new_unit();
    let xs = sphere.intersect(&ray);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], -4.0);
}

#[test]
fn test8_intersection_object_encapsulates_t_and_obj() {
    let distance = 3.5;
    let sphere = Sphere::new_unit();
    let intersection = Intersection::new(distance, &sphere);
    assert_eq!(intersection.distance, distance);
    assert_eq!(intersection.object, &sphere);
}

#[test]
fn test9_aggregating_intersections() {
    let s = Sphere::new_unit();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, &s);
    assert_eq!(xs[1].object, &s);
}
