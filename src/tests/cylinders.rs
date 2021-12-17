use crate::domain::object::{Cone, Cylinder};
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

#[test]
fn ch13_test8_normal_vector_on_cylinder_end_caps() {
    let cases = vec![
        (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
    ];

    let cyl = Cylinder::new()
        .minimum(1.0)
        .maximum(2.0)
        .closed(true)
        .build();

    for (point, normal_exp) in cases {
        let n = cyl.local_normal_at(&point);
        assert_eq!(n, normal_exp);
    }
}

#[test]
fn ch13_test9_intersecting_cone_with_ray() {
    let cases = vec![
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            5.0,
            5.0,
        ),
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(1.0, 1.0, 1.0),
            8.66025,
            8.66025,
        ),
        (
            Point::new(1.0, 1.0, -5.0),
            Vector::new(-0.5, -1.0, 1.0),
            4.55006,
            49.44994,
        ),
    ];

    let shape = Cone::new().build();
    for (origin, direction, t0, t1) in cases {
        let direction_n = direction.normalize();
        let r = Ray::new(origin, direction_n);
        let xs = shape.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::domain::epsilon_eq(xs[0], t0));
        assert!(crate::domain::epsilon_eq(xs[1], t1));
    }
}

#[test]
fn ch13_test10_intersecting_cone_with_ray_parallel_to_one_of_its_halves() {
    let shape = Cone::new().build();
    let direction = Vector::new(0.0, 1.0, 1.0).normalize();
    let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
    let xs = shape.local_intersect(&r);
    assert_eq!(xs.len(), 1);
    assert!(crate::domain::epsilon_eq(xs[0], 0.35355));
}

#[test]
fn ch13_test11_intersecting_cone_end_caps() {
    let cases = vec![
        (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
        (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
        (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
    ];

    let shape = Cone::new().minimum(-0.5).maximum(0.5).closed(true).build();
    for (origin, direction, count) in cases {
        let direction_n = direction.normalize();
        let r = Ray::new(origin, direction_n);
        let xs = shape.local_intersect(&r);
        assert_eq!(xs.len(), count);
    }
}

#[test]
fn ch13_test12_computing_normal_vector_on_cone() {
    let cases = [
        (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
        (
            Point::new(1.0, 1.0, 1.0),
            Vector::new(1.0, -2_f64.sqrt(), 1.0),
        ),
        (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
    ];

    let shape = Cone::new().build();
    for (point, normal_exp) in cases {
        let normal = shape.local_normal_at(&point);
        assert_eq!(normal, normal_exp);
    }
}
