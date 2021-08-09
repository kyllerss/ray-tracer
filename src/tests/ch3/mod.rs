#[cfg(test)]
mod tests {

    use crate::domain::canvas::*;
    use crate::domain::color::*;

    #[test]
    fn test1_build_canvas() {
        let black = Color::new(0.0, 0.0, 0.0);
        let c = Canvas::new(10, 20, &black);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for pixel in c.pixels {
            assert_eq!(*pixel, black);
        }
    }
}
