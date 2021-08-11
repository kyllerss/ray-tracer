#[cfg(test)]
mod tests {

    use crate::domain::canvas::Canvas;
    use crate::domain::color::*;
    use crate::utils::image_writer::*;
    use indoc::indoc;

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
    fn test3_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let r = c1 * c2;
        let exp = Color::new(0.9, 0.2, 0.04);
        assert_eq!(r, exp);
    }

    #[test]
    fn test4_build_canvas() {
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
    fn test5_write_pixel() {
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

    #[test]
    fn test6_construct_ppm_header() {
        let c = Canvas::new(5, 3, Color::default());

        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();
        let exp_ppm = indoc! {"P3
                               5 3
                               255
                              "};

        assert_eq!(ppm, exp_ppm);
    }

    #[test]
    fn test7_construct_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3, Color::default());
        let p1 = Color::new(1.5, 0.0, 0.0);
        let p2 = Color::new(0.0, 0.5, 0.0);
        let p3 = Color::new(-0.5, 0.0, 1.0);

        c[0][0] = p1;
        c[2][1] = p2;
        c[4][2] = p3;

        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();
        let exp_ppm = indoc! {"
            P3
            5 3
            255
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
        "};

        println!("{}", ppm);
        assert_eq!(ppm, exp_ppm);
    }
}
