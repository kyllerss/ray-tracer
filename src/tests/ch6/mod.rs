use crate::domain::object::Sphere;
use crate::domain::{Point, Vector};

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
