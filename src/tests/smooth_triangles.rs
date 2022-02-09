use crate::domain::object::{Object, SmoothTriangle};
use crate::domain::ray::Ray;
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
    assert_eq!(st.p1(), Point::new(0.0, 1.0, 0.0));
    assert_eq!(st.p2(), Point::new(-1.0, 0.0, 0.0));
    assert_eq!(st.p3(), Point::new(1.0, 0.0, 0.0));
    assert_eq!(st.n1, Vector::new(0.0, 1.0, 0.0));
    assert_eq!(st.n2, Vector::new(-1.0, 0.0, 0.0));
    assert_eq!(st.n3, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn ch15_test15_intersection_with_smooth_triangle_stores_u_v() {
    let st = build_smooth_triangle();
    let st_wrapped: Object = st.clone().into();
    let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
    let xs = st.local_intersect(&r, &st_wrapped);
    assert!(!xs.is_empty());
    assert!(xs[0].u.is_some());
    assert!(xs[0].v.is_some());
    assert!(crate::domain::epsilon_eq(xs[0].u.unwrap(), 0.45));
    assert!(crate::domain::epsilon_eq(xs[0].v.unwrap(), 0.25));
}