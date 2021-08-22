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
