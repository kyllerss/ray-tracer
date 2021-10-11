#[cfg(test)]
#[rustfmt::skip::macros(vec, matrix)]
mod tests {
    use crate::domain::matrix::Matrix;
    use crate::domain::{Point, Vector};
    use std::f64::consts::PI;

    #[test]
    fn ch4_test1_multiply_point_by_translation_matrix() {
        let t = Matrix::new_translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let r = &t * &p;
        let exp = Point::new(2.0, 1.0, 7.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test2_multiply_by_inverse_of_translation_matrix() {
        let t_inv = Matrix::new_translation(5.0, -3.0, 2.0).inverse();
        assert!(t_inv.is_some());
        let p = Point::new(-3.0, 4.0, 5.0);
        let r = &t_inv.unwrap() * &p;
        let exp = Point::new(-8.0, 7.0, 3.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test3_translation_does_not_affect_vectors() {
        let t = Matrix::new_translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        let r = &t * &v;
        assert_eq!(r, v);
    }

    #[test]
    fn ch4_test4_scaling_matrix_applied_to_point() {
        let t = Matrix::new_scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        let r = &t * &p;
        let exp = Point::new(-8.0, 18.0, 32.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test5_scaling_matrix_applied_to_vector() {
        let t = Matrix::new_scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let r = &t * &v;
        let exp = Vector::new(-8.0, 18.0, 32.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test6_multiplying_by_inverse_of_scaling_matrix() {
        let t_inv = Matrix::new_scaling(2.0, 3.0, 4.0).inverse();
        assert!(t_inv.is_some());
        let v = Vector::new(-4.0, 6.0, 8.0);
        let r = &t_inv.unwrap() * &v;
        let exp = Vector::new(-2.0, 2.0, 2.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test7_reflection_is_scaling_by_a_negative_value() {
        let t = Matrix::new_scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &t * &p;
        let exp = Point::new(-2.0, 3.0, 4.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test8_rotating_point_around_x_axis() {
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
    fn ch4_test9_inverse_of_x_rotation_rotates_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let t_half_quarter = Matrix::new_rotation_x(PI / 4 as f64);
        let t_inv_hq = t_half_quarter.inverse();
        assert!(t_inv_hq.is_some());
        let r = &t_inv_hq.unwrap() * &p;
        let exp = Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test10_rotating_point_around_y_axis() {
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
    fn ch4_test11_rotating_point_around_z_axis() {
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

    #[test]
    fn ch4_test12_and_13_shearing_transformations() {
        // x in proportion to y
        let m = Matrix::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(5.0, 3.0, 4.0);
        assert_eq!(r, exp);

        // x in proportion to z
        let m = Matrix::new_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(6.0, 3.0, 4.0);
        assert_eq!(r, exp);

        // y in proportion to x
        let m = Matrix::new_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(2.0, 5.0, 4.0);
        assert_eq!(r, exp);

        // y in proportion to z
        let m = Matrix::new_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(2.0, 7.0, 4.0);
        assert_eq!(r, exp);

        // z in proportion to x
        let m = Matrix::new_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(2.0, 3.0, 6.0);
        assert_eq!(r, exp);

        // z in proportion to y
        let m = Matrix::new_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let r = &m * &p;
        let exp = Point::new(2.0, 3.0, 7.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch4_test_14_chain_transformations() {
        // sequential transformations
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::new_rotation_x(PI / 2.0);
        let b = Matrix::new_scaling(5.0, 5.0, 5.0);
        let c = Matrix::new_translation(10.0, 5.0, 7.0);

        let p2 = &a * &p;
        let p3 = &b * &p2;
        let p4 = &c * &p3;

        let exp = Point::new(15.0, 0.0, 7.0);
        assert_eq!(p4, exp);

        // chained transformations
        let t = &(&c * &b) * &a;
        let p5 = &t * &p;
        assert_eq!(p5, exp);
    }

    #[test]
    fn ch7_test10_view_transformation_matrix_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::new_view_transformation(&from, &to, &up);
        let t_exp = crate::domain::matrix::IDENTITY.clone();
        assert_eq!(t, t_exp);
    }

    #[test]
    fn ch7_test11_view_transformation_matrix_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::new_view_transformation(&from, &to, &up);
        let t_exp = Matrix::new_scaling(-1.0, 1.0, -1.0);
        assert_eq!(t, t_exp);
    }

    #[test]
    fn ch7_test12_view_transformation_moves_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::new_view_transformation(&from, &to, &up);
        let t_exp = Matrix::new_translation(0.0, 0.0, -8.0);
        assert_eq!(t, t_exp);
    }

    #[test]
    #[rustfmt::skip::macros(vec)]
    fn ch7_test13_view_transformation_arbitrary() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Matrix::new_view_transformation(&from, &to, &up);
        let t_exp = Matrix::new(
            4,
            4,
            vec![-0.50709, 0.50709, 0.67612, -2.36643,
                 0.76772, 0.60609, 0.12122, -2.82843,
                 -0.35857, 0.59761, -0.71714, 0.0,
                 0.0, 0.0, 0.0, 1.0],
        );
        assert_eq!(t, t_exp);
    }
}
