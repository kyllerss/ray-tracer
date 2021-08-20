#[cfg(test)]
#[rustfmt::skip::macros(vec, matrix)]
mod tests {
    use crate::domain::matrix::Matrix;
    use crate::domain::{Point, Vector};
    use std::f64::consts::PI;

    #[test]
    fn test1_multiply_point_by_translation_matrix() {
        let t = Matrix::new_translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let r = &t * &p;
        let exp = Point::new(2.0, 1.0, 7.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test2_multiply_by_inverse_of_translation_matrix() {
        let t_inv = Matrix::new_translation(5.0, -3.0, 2.0).inverse();
        assert!(t_inv.is_some());
        let p = Point::new(-3.0, 4.0, 5.0);
        let r = &t_inv.unwrap() * &p;
        let exp = Point::new(-8.0, 7.0, 3.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test3_translation_does_not_affect_vectors() {
        let t = Matrix::new_translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        let r = &t * &v;
        assert_eq!(r, v);
    }

    #[test]
    fn test4_scaling_matrix_applied_to_point() {
        let t = Matrix::new_scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        let r = &t * &p;
        let exp = Point::new(-8.0, 18.0, 32.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test5_scaling_matrix_applied_to_vector() {
        let t = Matrix::new_scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let r = &t * &v;
        let exp = Vector::new(-8.0, 18.0, 32.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test6_multiplying_by_inverse_of_scaling_matrix() {
        let t_inv = Matrix::new_scaling(2.0, 3.0, 4.0).inverse();
        assert!(t_inv.is_some());
        let v = Vector::new(-4.0, 6.0, 8.0);
        let r = &t_inv.unwrap() * &v;
        let exp = Vector::new(-2.0, 2.0, 2.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test7_reflection_is_scaling_by_a_negative_value() {
        let t = Matrix::new_scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &t * &p;
        let exp = Point::new(-2.0, 3.0, 4.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test8_rotating_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let t_half_quarter = Matrix::new_rotation_x(PI / 4 as f64);
        let r = &t_half_quarter * &p;
        let exp = Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        assert_eq!(r, exp);

        let t_full_quarter = Matrix::new_rotation_x(PI / 2 as f64);
        let r = &t_full_quarter * &p;
        let exp = Point::new(0.0, 0.0, 1.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test9_inverse_of_x_rotation_rotates_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let t_half_quarter = Matrix::new_rotation_x(PI / 4 as f64);
        let t_inv_hq = t_half_quarter.inverse();
        assert!(t_inv_hq.is_some());
        let r = &t_inv_hq.unwrap() * &p;
        let exp = Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test10_rotating_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let t_half_quarter = Matrix::new_rotation_y(PI / 4 as f64);
        let r = &t_half_quarter * &p;
        let exp = Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0);
        assert_eq!(r, exp);

        let t_full_quarter = Matrix::new_rotation_y(PI / 2 as f64);
        let r = &t_full_quarter * &p;
        let exp = Point::new(1.0, 0.0, 0.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test11_rotating_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let t_half_quarter = Matrix::new_rotation_z(PI / 4 as f64);
        let r = &t_half_quarter * &p;
        let exp = Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
        assert_eq!(r, exp);

        let t_full_quarter = Matrix::new_rotation_z(PI / 2 as f64);
        let r = &t_full_quarter * &p;
        let exp = Point::new(-1.0, 0.0, 0.0);
        assert_eq!(r, exp);
    }
}
