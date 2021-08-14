#[cfg(test)]
mod tests {

    use crate::domain::canvas::Canvas;
    use crate::domain::color::*;
    use crate::domain::Point;
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
        for col in 0..width {
            for row in 0..height {
                //let row: &[Color] = &c[w];
                //let p: &Color = &row[h];
                let p: &Color = &c[row][col];
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

        let p_col: usize = 2;
        let p_row: usize = 3;
        c[p_row][p_col] = red;

        // verify all pixels
        for col in 0..width {
            for row in 0..height {
                let p: &Color = &c[row][col];
                let exp_p: Color;
                if col == p_col && row == p_row {
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
        let exp_ppm = indoc! {"
            P3
            5 3
            255
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 0 0 0 0
            "};
        //println!("{}", ppm);
        assert_eq!(ppm, exp_ppm);
    }

    #[test]
    fn test7_construct_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3, Color::default());
        let p1 = Color::new(1.5, 0.0, 0.0);
        let p2 = Color::new(0.0, 0.5, 0.0);
        let p3 = Color::new(-0.5, 0.0, 1.0);

        c[0][0] = p1;
        c[1][2] = p2;
        c[2][4] = p3;

        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();
        // let exp_ppm = indoc! {"
        //     P3
        //     5 3
        //     255
        //     255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
        //     0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
        // "};

        //println!("{}", ppm);
        let exp_ppm = indoc! {"
            P3
            5 3
            255
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 0 0 0 255
            "};
        assert_eq!(ppm, exp_ppm);
    }

    #[test]
    fn test8_no_lines_exceed_70_chars() {
        let default_color = Color::new(1.0, 0.8, 0.6);
        let c = Canvas::new(10, 2, default_color);
        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();

        //println!("{}", ppm);

        assert!(!ppm.is_empty());

        let lines = ppm.lines();
        for line in lines {
            assert!(line.len() < 70);
        }
    }

    #[test]
    fn test9_ppm_terminated_by_new_line_character() {
        let c = Canvas::new(10, 2, Color::default());
        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();

        assert!(!ppm.is_empty());
        assert!(ppm.ends_with("\n"));
    }

    #[test]
    fn test_pit_1_invert_y_coord() {
        let mut c = Canvas::new(10, 10, Color::default());
        let color = Color::new(1.0, 0.0, 0.0);
        let row = 2;
        let col = 2;
        let point = Point::new(col as f64, row as f64, 0.0);
        c.render(point, color);
        assert_eq!(c[row][col], color);

        c.invert_y();
        let exp_row = 8;
        let exp_col = 2;
        assert_eq!(c[row][col], Color::default());
        assert_eq!(c[exp_row][exp_col], color);
    }

    // #[test]
    // // [BUG] Canvases that are wider than taller don't render pixels consistently.
    // fn test_pit_1_width_longer_than_height_rendering_bug() {
    //     let width: usize = 10;
    //     let height: usize = 5;
    //     let mut c = Canvas::new(width, height, Color::default());
    //     let color = Color::new(1.0, 0.0, 0.0);
    //
    //     c.render(Point::new(0.0, 0.0, 0.0), color);
    //     c.render(Point::new(1.0, 1.0, 0.0), color);
    //     c.render(Point::new(2.0, 2.0, 0.0), color);
    //     c.render(Point::new(3.0, 3.0, 0.0), color);
    //     c.render(Point::new(4.0, 4.0, 0.0), color);
    //     c.render(Point::new(5.0, 5.0, 0.0), color);
    //     c.render(Point::new(6.0, 6.0, 0.0), color);
    //     c.render(Point::new(7.0, 7.0, 0.0), color);
    //     c.render(Point::new(8.0, 8.0, 0.0), color);
    //     c.render(Point::new(9.0, 9.0, 0.0), color);
    //
    //     println!("---------------");
    //     for (i, pixel) in c.into_iter().enumerate() {
    //         if i % 10 == 0 {
    //             println!("")
    //         };
    //         let hit = *pixel == color;
    //         print!("{}", if hit { 1 } else { 0 });
    //     }
    //     println!("");
    // }
}
