#[cfg(test)]
mod tests {

    use crate::domain::*;

    #[test]
    fn ch1_test1_build_and_validate_type_point() {

        let _x = 1.1;
        let _y = -2.2;
        let _z = 3.3;
        let p = Point::new(_x, _y, _z);
        assert_eq!(p.to_tuple(), (_x, _y, _z, 1.0));
    }

    #[test]
    fn ch1_test1_build_and_validate_type_vector() {

        let _x = 1.1;
        let _y = -2.2;
        let _z = 3.3;
        let p = Vector::new(_x, _y, _z);
        assert_eq!(p.to_tuple(), (_x, _y, _z, 0.0));
    }

}