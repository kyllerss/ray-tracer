#[cfg(test)]
mod tests {

    use crate::domain::color::Color;
    use crate::domain::*;

    #[test]
    fn ch1_test1_and_2_build_and_validate_type_point() {
        let _x = 1.1;
        let _y = -2.2;
        let _z = 3.3;
        let p = Point::new(_x, _y, _z);
        assert_eq!(p.to_ray_tuple(), (_x, _y, _z, 1.0));

        // test equality
        let a = Point::new(1.0000001, 2.00000002, 3.000000003);
        let b = Point::new(1.000000001, 2.00000002, 3.00000003);
        assert_eq!(a, b);

        let c = Point::new(1.01, 2.02, 3.03);
        let d = Point::new(1.0, 2.0, 3.0);
        assert_ne!(c, d);
    }

    #[test]
    fn ch1_test1_and_2_build_and_validate_type_vector() {
        let _x = 1.1;
        let _y = -2.2;
        let _z = 3.3;
        let p = Vector::new(_x, _y, _z);
        assert_eq!(p.to_ray_tuple(), (_x, _y, _z, 0.0));

        // test equality
        let a = Vector::new(1.0000001, 2.00000002, 3.000000003);
        let b = Vector::new(1.000000001, 2.00000002, 3.00000003);
        assert_eq!(a, b);

        let c = Vector::new(1.01, 2.02, 3.03);
        let d = Vector::new(1.0, 2.0, 3.0);
        assert_ne!(c, d);
    }

    #[test]
    fn ch1_test3_adding_two_tuples() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        // add point to vector
        let p1 = &p + &v;
        let expected = Point::new(1.0, 1.0, 6.0);
        assert_eq!(p1, expected);

        // test commutative property
        let p2 = &v + &p;
        assert_eq!(p2, expected);
    }

    #[test]
    fn ch1_test4_subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        let r = &p1 - &p2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test5_subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        let r = &p - &v;
        let exp = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test6_subtracting_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        let r = &v1 - &v2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test7_negating_vectors() {
        let v_pos = Vector::new(1.0, -2.0, 3.0);
        let v_neg = -v_pos;

        let exp = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(v_neg, exp);
    }

    #[test]
    fn ch1_test8_multiply_by_scale() {
        // scalar
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let scalar = 3.5 as f64;

        let r = &v1 * scalar;
        let exp = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(r, exp);

        // fraction
        let fraction = 0.5 as f64;
        let r2 = &v1 * fraction;
        let exp2 = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(r2, exp2);
    }

    #[test]
    fn ch1_test9_divide_by_scalar() {
        // scalar
        let v = Vector::new(1.0, -2.0, 3.0);
        let scalar = 2.0 as f64;

        let r = &v / scalar;
        let exp = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test10_compute_magnitude_of_vector() {
        let v = Vector::new(1.0, 0.0, 0.0);
        let m = v.magnitude();
        let exp = 1.0;
        assert_eq!(m, exp);

        let v = Vector::new(0.0, 1.0, 0.0);
        let m = v.magnitude();
        let exp = 1.0;
        assert_eq!(m, exp);

        let v = Vector::new(0.0, 0.0, 1.0);
        let m = v.magnitude();
        let exp = 1.0;
        assert_eq!(m, exp);

        let v = Vector::new(1.0, 2.0, 3.0);
        let m = v.magnitude();
        let exp = f64::sqrt(14.0);
        assert_eq!(m, exp);

        let v = Vector::new(-1.0, -2.0, -3.0);
        let m = v.magnitude();
        let exp = f64::sqrt(14.0);
        assert_eq!(m, exp);
    }

    #[test]
    fn ch1_test11_normalize_vectors() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let n = v.normalize();
        let exp = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(n, exp);

        let v = Vector::new(1.0, 2.0, 3.0);
        let n = v.normalize();
        let exp = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(n, exp);

        let v = Vector::new(1.0, 2.0, 3.0);
        let mag = v.normalize().magnitude();
        let exp_mag = 1.0;
        assert_eq!(mag, exp_mag);
    }

    #[test]
    fn ch1_test12_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let dot = v1.dot_product(&v2);
        let exp = 20.0;
        assert_eq!(dot, exp);
    }

    #[test]
    fn ch1_test13_cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let cross_product = v1.cross_product(&v2);
        let exp = Vector::new(-1.0, 2.0, -1.0);
        assert_eq!(cross_product, exp);

        let cross_product = v2.cross_product(&v1);
        let exp = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(cross_product, exp);
    }

    #[test]
    fn ch2_test1_build_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn ch2_test2_and_3_color_manipulation() {
        // adding colors
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let r = &c1 + &c2;
        let exp = Color::new(1.6, 0.7, 1.0);
        assert_eq!(r, exp);

        // subtracting colors
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let r = &c1 - &c2;
        let exp = Color::new(0.2, 0.5, 0.5);
        assert_eq!(r, exp);

        // multiplying by scalar
        let c = Color::new(0.2, 0.3, 0.4);
        let r = &c * 2.0;
        let exp = Color::new(0.4, 0.6, 0.8);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch2_test3_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let r = &c1 * &c2;
        let exp = Color::new(0.9, 0.2, 0.04);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch6_test4_reflecting_vector_at_45_deg() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        let exp_r = Vector::new(1.0, 1.0, 0.0);

        assert_eq!(r, exp_r);
    }

    #[test]
    fn ch6_test5_reflecting_vector_at_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(&n);
        let exp_r = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(r, exp_r);
    }
}
