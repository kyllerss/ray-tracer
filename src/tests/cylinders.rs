use crate::domain::object::Cylinder;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch13_test1_ray_misses_cylinder() {
    // test cases
    let cases = [
        (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
    ];

    // test code
    let cyl = Cylinder::new().build();
    for (origin, direction) in cases {
        let direction_normalized = direction.normalize();
        let r = Ray::new(origin, direction_normalized);
        let xs = cyl.local_intersect(&r);
        assert!(xs.is_empty());
    }
}

#[test]
fn ch13_test2_ray_strikes_cylinder() {
    // test cases
    let cases = [
        (
            Point::new(1.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            5.0,
            5.0,
        ),
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.1, 1.0, 1.0),
            6.80798,
            7.08872,
        ),
    ];

    // test code
    let cyl = Cylinder::new().build();
    for (origin, direction, t0, t1) in cases {
        let direction_normalized = direction.normalize();
        let r = Ray::new(origin, direction_normalized);
        let xs = cyl.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::domain::epsilon_eq(xs[0], t0));
        assert!(crate::domain::epsilon_eq(xs[1], t1));
    }
}

#[test]
fn ch13_test3_normal_vector_on_cylinder() {
    let cases = [
        (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
        (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
    ];

    let cyl = Cylinder::new().build();
    for (point, n_exp) in cases {
        let n = cyl.local_normal_at(&point);
        assert_eq!(n, n_exp);
    }
}

#[test]
fn ch13_test4_default_minimum_and_maximum_for_cylinder() {
    let cyl = Cylinder::new().build();
    assert_eq!(cyl.minimum, -f64::INFINITY);
    assert_eq!(cyl.maximum, f64::INFINITY);
}

#[test]
fn ch13_test5_intersecting_constrained_cylinder() {
    let cases = vec![
        (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
        (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
    ];

    let cyl = Cylinder::new().minimum(1.0).maximum(2.0).build();
    for (point, direction, count) in cases {
        let direction_n = direction.normalize();
        let r = Ray::new(point, direction_n);
        let xs = cyl.local_intersect(&r);
        assert_eq!(xs.len(), count);
    }
}

#[test]
fn ch13_test6_default_closed_value_for_cylinder() {
    let cyl = Cylinder::new().build();
    assert_eq!(cyl.closed, false);
}

#[test]
fn ch13_test7_intersecting_caps_of_closed_cylinder() {
    let cases = [
        (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
        (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
        (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
        (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
        (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
    ];

    let cyl = Cylinder::new()
        .minimum(1.0)
        .maximum(2.0)
        .closed(true)
        .build();
    for (point, direction, count) in cases {
        let direction_n = direction.normalize();
        let r = Ray::new(point, direction_n);
        let xs = cyl.local_intersect(&r);
        assert_eq!(xs.len(), count);
    }
}
