#[cfg(test)]
mod tests {

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
    fn ch1_test4_subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        let r = p1 - p2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test5_subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        let r = p - v;
        let exp = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }

    #[test]
    fn ch1_test6_subtracting_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        let r = v1 - v2;
        let exp = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(r, exp);
    }
}