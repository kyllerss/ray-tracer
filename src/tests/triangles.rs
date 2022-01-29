use crate::domain::object::Triangle;
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
