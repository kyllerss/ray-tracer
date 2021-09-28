use crate::domain::matrix::Matrix;
use crate::domain::object::Sphere;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

#[test]
fn test1_normal_on_a_sphere() {
    // normal on an x-axis
    let s = Sphere::new_unit();
    let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
    let exp_n = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a y-axis
    let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
    let exp_n = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(n, exp_n);

    // normal on a z-axis
    let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
    let exp_n = Vector::new(0.0, 0.0, 1.0);
    assert_eq!(n, exp_n);

    // normal on a nonaxial point
    let n = s.normal_at(Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let exp_n = Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0);
    assert_eq!(n, exp_n);
}

#[test]
fn test2_normal_is_normalized() {
    let s = Sphere::new_unit();
    let n = s.normal_at(Point::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let normalized = n.normalize();

    assert_eq!(n, normalized);
}

#[test]
fn test3_computing_normal_of_modified_sphere() {
    // translated sphere
    let t = Matrix::new_translation(0.0, 1.0, 0.0);
    let s = Sphere::new(t);
    let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
    let exp_n = Vector::new(0.0, 0.70711, -0.70711);

    assert_eq!(n, exp_n);

    let t = &Matrix::new_scaling(1.0, 0.5, 1.0) * &Matrix::new_rotation_z(PI / 0.5);
    let s = Sphere::new(t);
    let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
    let exp_n = Vector::new(0.0, 0.97014, -0.24254);

    assert_eq!(n, exp_n);
}
