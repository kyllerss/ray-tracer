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
