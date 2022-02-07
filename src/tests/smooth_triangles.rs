use crate::domain::object::SmoothTriangle;
use crate::domain::{Point, Vector};

fn build_smooth_triangle<'a>() -> SmoothTriangle<'a> {
    let p1 = Point::new(0.0, 1.0, 0.0);
    let p2 = Point::new(-1.0, 0.0, 0.0);
    let p3 = Point::new(1.0, 0.0, 0.0);
    let n1 = Vector::new(0.0, 1.0, 0.0);
    let n2 = Vector::new(-1.0, 0.0, 0.0);
    let n3 = Vector::new(1.0, 0.0, 0.0);

    SmoothTriangle::builder(p1, p2, p3, n1, n2, n3).build()
}

#[test]
fn ch15_test13_construct_smooth_triangle() {
    let st = build_smooth_triangle();
    assert_eq!(st.p1, Point::new(0.0, 1.0, 0.0));
    assert_eq!(st.p2, Point::new(-1.0, 0.0, 0.0));
    assert_eq!(st.p3, Point::new(1.0, 0.0, 0.0));
    assert_eq!(st.n1, Vector::new(0.0, 1.0, 0.0));
    assert_eq!(st.n2, Vector::new(-1.0, 0.0, 0.0));
    assert_eq!(st.n3, Vector::new(1.0, 0.0, 0.0));
}
