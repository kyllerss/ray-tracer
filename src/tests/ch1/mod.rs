#[cfg(test)]
mod tests {

    use crate::domain::*;

    #[test]
    fn test1_and_2_build_and_validate_type_point() {

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
    fn test1_and_2_build_and_validate_type_vector() {

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
    fn test3_adding_two_tuples() {

        let p = Point::new(3.0,-2.0,5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);

        // add point to vector
        let p1 = p + v;
        let expected = Point::new(1.0, 1.0, 6.0);
        assert_eq!(p1, expected);

        // test commutative property
        let p2 = v + p;
        assert_eq!(p2, expected);
    }

    #[test]
    fn test4_subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        let r = p1 - p2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test5_subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        let r = p - v;
        let exp = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test6_subtracting_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        let r = v1 - v2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn test7_negating_vectors() {
        let v_pos = Vector::new(1.0, -2.0, 3.0);
        let v_neg = -v_pos;

        let exp = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(v_neg, exp);
    }

    #[test]
    fn test8_multiply_by_scale() {

        // scalar
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let scalar = 3.5 as f64;

        let r = v1 * scalar;
        let exp = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(r, exp);

        // fraction
        let fraction = 0.5 as f64;
        let r2 = v1 * fraction;
        let exp2 = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(r2, exp2);
    }

    #[test]
    fn test9_divide_by_scalar() {

        // scalar
        let v = Vector::new(1.0, -2.0, 3.0);
        let scalar = 2.0 as f64;

        let r = v / scalar;
        let exp = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(r, exp);
    }

    #[test]
    fn test10_compute_magnitude_of_vector() {

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
    fn test11_normalize_vectors() {
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
    fn test12_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let dot = v1.dot_product(v2);
        let exp = 20.0;
        assert_eq!(dot, exp);
    }

    #[test]
    fn test13_cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let cross_product = v1.cross_product(v2);
        let exp = Vector::new(-1.0, 2.0, -1.0);
        assert_eq!(cross_product, exp);

        let cross_product = v2.cross_product(v1);
        let exp = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(cross_product, exp);
    }
}