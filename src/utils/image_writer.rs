use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use std::fmt::Write;

pub const PPM3_MAGIC_NUMBER: &str = "P3";
pub const PPM3_MAX_COLOR_VALUE: u8 = 255;
pub const PPM3_MAX_LINE_LENGTH: usize = 70;

pub enum Format {
    Ppm3,
}

pub struct ImageWriter<'a> {
    format: Format,
    canvas: &'a Canvas,
}

// Returns the ppm3 color value from normalized color value
fn ppm3_color_value(v: f32) -> String {
    if v <= 0.0 {
        return "0".to_string();
    }

    if v >= 1.0 {
        return PPM3_MAX_COLOR_VALUE.to_string();
    }

    let scaled_v: u8 = ((PPM3_MAX_COLOR_VALUE as f32) * v) as u8;
    scaled_v.to_string()
}

impl<'a> ImageWriter<'a> {
    // consructor
    pub fn new(format: Format, canvas: &'a Canvas) -> ImageWriter {
        ImageWriter { format, canvas }
    }

    // converts pixel to text representation
    pub fn to_string_encoding(&self, pixel: &Color) -> String {
        match &self.format {
            Format::Ppm3 => {
                let r_val = ppm3_color_value(pixel.red);
                let g_val = ppm3_color_value(pixel.green);
                let b_val = ppm3_color_value(pixel.blue);
                let r = format!("{} {} {}", r_val, g_val, b_val);
                r
            }
        }
    }

    // provides string representation of image
    pub fn to_string(&self) -> String {
        let mut ppm = String::new();

        // header
        let mut header = String::new();
        match &self.format {
            Format::Ppm3 => {
                writeln!(header, "{}", PPM3_MAGIC_NUMBER).unwrap();
                writeln!(
                    header,
                    "{} {}",
                    self.canvas.width.to_string(),
                    self.canvas.height.to_string()
                )
                .unwrap();
                writeln!(header, "{}", PPM3_MAX_COLOR_VALUE).unwrap();
            }
        }

        // body
        let mut body = String::new();
        let mut line_length: usize = 0;
        for pixel in self.canvas {
            let p_str = self.to_string_encoding(pixel);
            if p_str.len() + line_length >= PPM3_MAX_LINE_LENGTH {
                body.push('\n');
                line_length = 0;
            }

            if line_length != 0 {
                body.push(' ');
            }

            line_length += p_str.len();
            body.push_str(&*p_str);
        }
        // match &self.format {
        //     Format::Ppm3 => {
        //         let
        //     }
        // }

        // answer
        ppm.push_str(&*header);
        ppm.push_str(&*body);

        ppm
    }
}
