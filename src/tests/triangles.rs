use crate::domain::object::{Object, Triangle};
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

#[test]
fn ch15_test1_constructing_triangle() {
    let p1 = Point::new(0.0, 1.0, 0.0);
    let p2 = Point::new(-1.0, 0.0, 0.0);
    let p3 = Point::new(1.0, 0.0, 0.0);

    let t = Triangle::new(p1, p2, p3);

    assert_eq!(t.p1, p1);
    assert_eq!(t.p2, p2);
    assert_eq!(t.p3, p3);

    assert_eq!(t.e1, Vector::new(-1.0, -1.0, 0.0));
    assert_eq!(t.e2, Vector::new(1.0, -1.0, 0.0));
    assert_eq!(t.normal, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn ch15_test2_finding_normal_on_triangle() {
    let t = Triangle::new(
        Point::new(0.0, 1.0, 0.0),
        Point::new(-1.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    );
    let n1 = t.local_normal_at(&Point::new(0.0, 0.5, 0.0));
    let n2 = t.local_normal_at(&Point::new(-0.5, 0.75, 0.0));
    let n3 = t.local_normal_at(&Point::new(0.5, 0.25, 0.0));

    assert_eq!(n1, t.normal);
    assert_eq!(n2, t.normal);
    assert_eq!(n3, t.normal);
}

#[test]
fn ch15_test3_intersecting_ray_parallel_to_triangle() {
    let t: Object = Triangle::new(
        Point::new(0.0, 1.0, 0.0),
        Point::new(-1.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    )
    .into();

    let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 0.0));

    let xs = t.local_intersect(&r);
    assert!(xs.is_empty());
}
