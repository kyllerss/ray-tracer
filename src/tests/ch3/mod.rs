#[cfg(test)]
#[rustfmt::skip::macros(vec, matrix)]
mod tests {
    use crate::domain::matrix::Matrix;
    use crate::domain::{Point, Vector};

    #[test]
    fn test1_construct_and_inspect_4x4_matrix() {
        let m = matrix![1.0,  2.0,  3.0,  4.0;
                               5.5,  6.5,  7.5,  8.5;
                               9.0,  10.0, 11.0, 12.0;
                               13.5, 14.5, 15.5, 16.5];
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn test2_different_dimensions_support() {
        // 2x2
        let m = matrix![-3.0,  5.0;
                                1.0, -2.0];
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);

        // 3x3
        let m = matrix![-3.0,  5.0, 0.0;
                                1.0, -2.0, -7.0;
                                0.0, 1.0, 1.0];
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn test3_matrix_equality() {
        let a = matrix![1.0, 2.0, 3.0, 4.0;
                        5.0, 6.0, 7.0, 8.0;
                        9.0, 8.0, 7.0, 6.0;
                        5.0, 4.0, 3.0, 2.0];
        let b = matrix![1.0, 2.0, 3.0, 4.0;
                        5.0, 6.0, 7.0, 8.0;
                        9.0, 8.0, 7.0, 6.0;
                        5.0, 4.0, 3.0, 2.0];

        assert_eq!(a, b);

        let b = matrix![2.0, 3.0, 4.0, 5.0;
                        6.0, 7.0, 8.0, 9.0;
                        8.0, 7.0, 6.0, 5.0;
                        4.0, 3.0, 2.0, 1.0];

        assert_ne!(a, b);
    }

    #[test]
    fn test4_multiplying_two_matrices() {
        let a = &matrix![1.0, 2.0, 3.0, 4.0;
                        5.0, 6.0, 7.0, 8.0;
                        9.0, 8.0, 7.0, 6.0;
                        5.0, 4.0, 3.0, 2.0];
        let b = matrix![-2.0, 1.0, 2.0, 3.0;
                        3.0, 2.0, 1.0, -1.0;
                        4.0, 3.0, 6.0, 5.0;
                        1.0, 2.0, 7.0, 8.0];
        let exp = matrix![20.0, 22.0, 50.0, 48.0;
                          44.0, 54.0, 114.0, 108.0;
                          40.0, 58.0, 110.0, 102.0;
                          16.0, 26.0, 46.0, 42.0];

        let r = a * &b;
        assert_eq!(r, exp);
    }

    #[test]
    fn test5_multiplying_matrix_by_tuple() {
        let a: &Matrix = &matrix![1.0, 2.0, 3.0, 4.0;
                        2.0, 4.0, 4.0, 2.0;
                        8.0, 6.0, 4.0, 1.0;
                        0.0, 0.0, 0.0, 1.0];
        let point = Point::new(1.0, 2.0, 3.0);
        let exp = Point::new(18.0, 24.0, 33.0);

        let r = a * &point;
        assert_eq!(r, exp);

        // additional test for vectors
        let vector = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        let r = a * &vector;
        assert_eq!(r, exp);
    }

    #[test]
    fn test6_multiply_by_identify_matrix() {
        let a: &Matrix = &matrix![0.0, 1.0, 2.0, 4.0;
                                  1.0, 2.0, 4.0, 8.0;
                                  2.0, 4.0, 8.0, 16.0;
                                  4.0, 8.0, 16.0, 32.0];
        let a1 = a * &*crate::domain::matrix::IDENTITY;
        assert_eq!(*a, a1);
    }

    #[test]
    fn test7_transposing_matrix() {
        let mut a: Matrix = matrix![0.0, 9.0, 3.0, 0.0;
                                    9.0, 8.0, 0.0, 8.0;
                                    1.0, 8.0, 5.0, 3.0;
                                    0.0, 0.0, 5.0, 8.0];
        let exp: Matrix = matrix![0.0, 9.0, 1.0, 0.0;
                                  9.0, 8.0, 8.0, 0.0;
                                  3.0, 0.0, 5.0, 5.0;
                                  0.0, 8.0, 3.0, 8.0];

        a.transpose();
        assert_eq!(a, exp);
    }

    #[test]
    fn test8_transposing_identity_matrix() {
        let mut identity_clone = crate::domain::matrix::IDENTITY.clone();
        identity_clone.transpose();
        assert_eq!(identity_clone, *crate::domain::matrix::IDENTITY);
    }

    #[test]
    fn test9_determinant_of_2x2() {
        let m = matrix![1.0, 5.0;
                        -3.0, 2.0];
        let r: f64 = m.determinant();
        let exp = 17.0;

        assert_eq!(r, exp);
    }

    #[test]
    fn test10_calculate_submatrices() {
        // 3x3 matrix
        let m = matrix![1.0, 5.0, 0.0;
                        -3.0, 2.0, 7.0;
                        0.0, 6.0, -3.0];
        let r = m.submatrix(0, 2);
        let exp = matrix![-3.0, 2.0;
                          0.0, 6.0];
        assert_eq!(r, exp);

        // 4x4 matrix
        let m = matrix![-6.0, 1.0, 1.0, 6.0;
                        -8.0, 5.0, 8.0, 6.0;
                        -1.0, 0.0, 8.0, 2.0;
                        -7.0, 1.0, -1.0, 1.0];
        let r = m.submatrix(2, 1);
        let exp = matrix![-6.0, 1.0, 6.0;
                          -8.0, 8.0, 6.0;
                          -7.0, -1.0, 1.0];
        assert_eq!(r, exp);
    }

    #[test]
    fn test11_calculate_minor_of_3x3_matrix() {
        let m = matrix![3.0, 5.0, 0.0;
                        2.0, -1.0, -7.0;
                        6.0, -1.0, 5.0];
        let r = m.minor(1, 0);
        let exp = 25.0;
        assert_eq!(r, exp);
    }

    #[test]
    fn test12_calculate_cofactor_of_3x3_matrix() {
        let m = matrix![3.0, 5.0, 0.0;
                        2.0, -1.0, -7.0;
                        6.0, -1.0, 5.0];
        let r = m.cofactor(0, 0);
        let exp = -12.0;
        assert_eq!(r, exp);

        let r = m.cofactor(1, 0);
        let exp = -25.0;
        assert_eq!(r, exp);
    }

    #[test]
    fn test13_calculate_determinant_3x3_and_4x4_matrices() {
        // 3x3 test
        let m = matrix![1.0, 2.0, 6.0;
                        -5.0, 8.0, -4.0;
                        2.0, 6.0, 4.0];
        let r = m.determinant();
        let exp = -196.0;
        assert_eq!(r, exp);

        // 4x4 test
        let m = matrix![-2.0, -8.0, 3.0, 5.0;
                        -3.0, 1.0, 7.0, 3.0;
                        1.0, 2.0, -9.0, 6.0;
                        -6.0, 7.0, 7.0, -9.0];
        let r = m.determinant();
        let exp = -4071.0;
        assert_eq!(r, exp);
    }

    #[test]
    fn test14_invertability_of_matrices() {
        // invertible
        let m = matrix![6.0, 4.0, 4.0, 4.0;
                        5.0, 5.0, 7.0, 6.0;
                        4.0, -9.0, 3.0, -7.0;
                        9.0, 1.0, 7.0, -6.0];
        assert!(m.is_invertible());

        // not invertible
        let m = matrix![-4.0, 2.0, -2.0, -3.0;
                        9.0, 6.0, 2.0, 6.0;
                        0.0, -5.0, 1.0, -5.0;
                        0.0, 0.0, 0.0, 0.0];
        assert!(!m.is_invertible());
    }
}
