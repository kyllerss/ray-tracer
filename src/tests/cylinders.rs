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
