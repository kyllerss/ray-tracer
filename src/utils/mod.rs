pub mod image_writer;
pub(crate) mod obj_parser;

use crate::domain::canvas::Canvas;
use crate::utils::image_writer::{Format, ImageWriter};
use std::fmt::Display;
use std::fs;
use std::io::Error;
use std::time::SystemTime;

// Utility method to write to file given canvas
pub fn write_imagefile<T: Into<String> + Display>(
    target_filename: T,
    target_dir: T,
    canvas: &Canvas,
) -> Result<(), Error> {
    let writer = ImageWriter::new(Format::Ppm3, canvas);
    let ppm = writer.to_string();
    let timestamp = {
        let start = SystemTime::now();
        start.duration_since(SystemTime::UNIX_EPOCH)
    }
    .expect("Unable to calculate system time.")
    .as_millis();
    let filename = format!("{}/{}_{}.ppm", target_dir, target_filename, timestamp);
    fs::write(filename, ppm)?;
    Ok(())
}
