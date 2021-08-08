#[cfg(test)]
mod tests {

    use crate::domain::color::*;

    #[test]
    fn test1_build_color() {

        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test2_and_3_color_manipulation() {

        // adding colors
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let r = c1 + c2;
        let exp = Color::new(1.6, 0.7, 1.0);
        assert_eq!(r, exp);

        // subtracting colors
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let r = c1 - c2;
        let exp = Color::new(0.2, 0.5, 0.5);
        assert_eq!(r, exp);

        // multiplying by scalar
        let c = Color::new(0.2, 0.3, 0.4);
        let r = c * 2.0;
        let exp = Color::new(0.4, 0.6, 0.8);
        assert_eq!(r, exp);
    }

    #[test]
    fn test4_multiplying_colors() {

        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let r = c1 * c2;
        let exp = Color::new(0.9, 0.2, 0.04);
        assert_eq!(r, exp);
    }
}