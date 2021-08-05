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

}