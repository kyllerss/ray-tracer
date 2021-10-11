use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch5_test1_creating_querying_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn ch5_test2_computing_point_from_distance() {
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
fn ch5_test12_translating_and_scaling_a_ray() {
    // translate ray
    let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    let m = Matrix::new_translation(3.0, 4.0, 5.0);
    let ray_2 = ray.transform(&m);

    assert_eq!(ray_2.origin, Point::new(4.0, 6.0, 8.0));
    assert_eq!(ray_2.direction, Vector::new(0.0, 1.0, 0.0));

    // scale ray
    let m = Matrix::new_scaling(2.0, 3.0, 4.0);
    let ray_2 = ray.transform(&m);

    assert_eq!(ray_2.origin, Point::new(2.0, 6.0, 12.0));
    assert_eq!(ray_2.direction, Vector::new(0.0, 3.0, 0.0));
}
