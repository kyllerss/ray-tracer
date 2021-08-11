#[cfg(test)]
mod tests {

    use crate::domain::canvas::*;
    use crate::domain::color::*;

    #[test]
    fn test1_build_canvas() {
        let black = Color::new(0.0, 0.0, 0.0);
        //let green = Color::new(0.0, 1.0, 0.0);
        let mut c = Canvas::new(10, 20, black);
        let width: usize = 10;
        let height: usize = 20;

        assert_eq!(c.width, width);
        assert_eq!(c.height, height);

        assert_eq!(c.pixels.is_empty(), false);

        // verify by iterator
        let mut iterated = false;
        for pixel in &c {
            assert_eq!(*pixel, black);
            iterated = true;
        }
        assert!(iterated);

        // verify by index lookup
        for w in 0..width {
            for h in 0..height {
                // let row: &[Color] = &c[w];
                // let cell = row[h];
                let p: &Color = &c[w][h];
                assert_eq!(*p, black);
            }
        }
    }
}
