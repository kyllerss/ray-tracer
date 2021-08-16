#[cfg(test)]
#[rustfmt::skip::macros(vec, matrix)]
mod tests {

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
}
