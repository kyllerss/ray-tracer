use crate::domain::canvas::Canvas;
use std::fmt::Write;

pub enum Format {
    Ppm3,
}

pub struct ImageWriter<'a> {
    format: Format,
    canvas: &'a Canvas,
}

impl<'a> ImageWriter<'a> {
    // consructor
    pub fn new(format: Format, canvas: &'a Canvas) -> ImageWriter {
        ImageWriter { format, canvas }
    }

    // provides string representation of image
    pub fn to_string(&self) -> String {
        let mut ppm = String::new();

        // header
        match &self.format {
            Format::Ppm3 => {
                writeln!(ppm, "P3").unwrap();
                writeln!(
                    ppm,
                    "{} {}",
                    self.canvas.width.to_string(),
                    self.canvas.height.to_string()
                )
                .unwrap();
                writeln!(ppm, "255").unwrap();
            } // _ => {
              //     panic!("Unimplemented file format")
              // }
        }

        // body

        // answer
        ppm
    }
}
