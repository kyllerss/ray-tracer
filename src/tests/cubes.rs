use crate::domain::object::Cube;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch12_test1_ray_intersects_cube() {
    let c = Cube::new().build();

    // +x
    let r = Ray::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // -x
    let r = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // +y
    let r = Ray::new(Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // -y
    let r = Ray::new(Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // +z
    let r = Ray::new(Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // -z
    let r = Ray::new(Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);

    // inside
    let r = Ray::new(Point::new(0.5, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
    let xs = c.local_intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}

#[test]
fn ch12_test2_ray_misses_cube() {
    let c = Cube::new().build();

    let cases = vec![
        (
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.2673, 0.5345, 0.8018),
        ),
        (
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.8018, 0.2673, 0.5345),
        ),
        (
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.5345, 0.8018, 0.2673),
        ),
        (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
    ];

    for (origin, direction) in cases {
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r);
        assert!(xs.is_empty());
    }
}

#[test]
fn ch12_test3_normal_on_surface_of_cube() {
    let cases = vec![
        (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
        (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
        (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
    ];

    let c = Cube::new().build();
    for (point, normal_exp) in cases {
        let normal = c.local_normal_at(&point);
        assert_eq!(normal, normal_exp);
    }
}
