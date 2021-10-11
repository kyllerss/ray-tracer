#[cfg(test)]
mod tests {

    use crate::domain::canvas::Canvas;
    use crate::domain::color::*;
    use crate::domain::Point;
    use crate::utils::image_writer::*;
    use indoc::indoc;
    use std::fmt::Write;

    #[test]
    fn ch2_test4_build_canvas() {
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
    fn ch2_test5_write_pixel() {
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
    fn ch2_test6_construct_ppm_header() {
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
    fn ch2_test7_construct_ppm_pixel_data() {
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
    fn ch2_test8_no_lines_exceed_70_chars() {
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
    fn ch2_test9_ppm_terminated_by_new_line_character() {
        let c = Canvas::new(10, 2, Color::default());
        let writer = ImageWriter::new(Format::Ppm3, &c);
        let ppm = writer.to_string();

        assert!(!ppm.is_empty());
        assert!(ppm.ends_with("\n"));
    }

    #[test]
    fn ch2_test_pit_1_invert_y_coord() {
        let mut c = Canvas::new(3, 3, Color::default());
        let color = Color::new(1.0, 0.0, 0.0);
        let row = 0;
        let col = 2;
        let point = Point::new(col as f64, row as f64, 0.0);
        c.render(
            point.x().round() as usize,
            point.y().round() as usize,
            color,
        );
        assert_eq!(c[row][col], color);

        c.invert_y();
        let exp_row = 2;
        let exp_col = 2;
        assert_eq!(c[row][col], Color::default());
        assert_eq!(c[exp_row][exp_col], color);
    }

    #[test]
    // [BUG] Canvases that are wider than taller don't render pixels consistently.
    fn ch2_test_pit_1_width_longer_than_height_rendering_bug() {
        let width: usize = 10;
        let height: usize = 5;
        let mut c = Canvas::new(width, height, Color::default());
        let color = Color::new(1.0, 0.0, 0.0);

        c.render(0, 0, color);
        c.render(1, 1, color);
        c.render(2, 2, color);
        c.render(3, 3, color);
        c.render(4, 4, color);
        c.render(5, 5, color);
        c.render(6, 6, color);
        c.render(7, 7, color);
        c.render(8, 8, color);
        c.render(9, 9, color);

        let result = print_grid(&c, color);
        let exp = indoc! {"
            1000000000
            0100000000
            0010000000
            0001000000
            0000100000"};
        // println!("----ORG----");
        // println!("{}", result);
        assert_eq!(result, exp);

        c.invert_y();
        let result = print_grid(&c, color);
        let exp = indoc! {"
            0000100000
            0001000000
            0010000000
            0100000000
            1000000000"};
        // println!("----INV----");
        // println!("{}", result);
        assert_eq!(result, exp);
    }

    fn print_grid(c: &Canvas, hit_color: Color) -> String {
        let mut r = String::new();
        for (i, pixel) in c.into_iter().enumerate() {
            if i % c.width == 0 && i != 0 {
                writeln!(r, "").unwrap();
            };
            let hit = *pixel == hit_color;
            write!(r, "{}", if hit { 1 } else { 0 }).unwrap();
        }
        r
    }
}
