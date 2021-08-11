#[cfg(test)]
mod tests {

    use crate::domain::canvas::*;
    use crate::domain::color::*;

    #[test]
    fn test1_build_canvas() {

        let width: usize = 10;
        let height: usize = 20;
        let black = Color::new(0.0, 0.0, 0.0);
        let c = Canvas::new(width, height, black);

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
                //let row: &[Color] = &c[w];
                //let p: &Color = &row[h];
                let p: &Color = &c[w][h];
                assert_eq!(*p, black);
            }
        }
    }

    #[test]
    fn test2_write_pixel() {

        let width: usize = 10;
        let height: usize = 20;
        let black = Color::new(0.0, 0.0, 0.0);
        let red = Color::new(1.0, 0.0, 0.0);
        let mut c = Canvas::new(width, height, black);

        let p_x: usize = 2;
        let p_y: usize = 3;
        c[p_x][p_y] = red;

        // verify all pixels
        for x in 0..width {
            for y in 0..height {

                let p: &Color = &c[x][y];
                let exp_p: Color;
                if x == p_x && y == p_y {
                    exp_p = red;
                } else {
                    exp_p = black;
                }

                assert_eq!(*p, exp_p);
            }
        }
    }
}
