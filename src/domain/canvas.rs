//use std::ops::{Index, IndexMut};
use std::ops::{Index, IndexMut};

use crate::domain::color::Color;
use crate::domain::Point;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    // constructor
    pub fn new(width: usize, height: usize, default_color: Color) -> Canvas {
        let vec_length = width * height;
        let pixels = vec![default_color; vec_length];

        Canvas {
            width,
            height,
            pixels,
        }
    }

    // renders a given point with a given color applying all necessary coord translations
    pub fn render(&mut self, point: Point, color: Color) {
        let column = point.x().round() as usize;
        let row = point.y().round() as usize;

        if column >= self.width || row >= self.height {
            return;
        }

        //println!("Coords ({}, {})", column, row);

        self[row][column] = color;
    }

    // for rendering purposes, inverts y coordinates to ensure upper-left coord system
    // translated to bottom-left coord system.
    pub fn invert_y(&mut self) {
        // iterate across half of pixels - leave odd-numbered middle row untouched
        let mid_row = f64::floor(self.height as f64 / 2.0) as usize;

        let mid_idx = mid_row * self.width;
        for idx in 0..mid_idx + 1 {
            let row = (idx / self.width) as usize;
            let col = (idx % self.width) as usize;
            let row_inv = (self.height - 1) - row;
            let inv_idx = (row_inv * self.width) + col;
            self.pixels.swap(idx, inv_idx);
        }

        // for row_idx in 0..(mid_row + 1) {
        //     let inv_row_idx = (self.height - 1) - row_idx;
        //     let row = &mut self[row_idx];
        //     let row_inv = &mut self[inv_row_idx];
        //     for col in 0..self.width {
        //         swap(&mut row[col], &mut row_inv[col]);
        //     }
        // }
    }

    // returns pixel at given coordinates
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self[x][y]
    }
}

impl Index<usize> for Canvas {
    type Output = [Color];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        let p = &self.pixels;
        let r: &[Color] = &p[start..start + self.width];
        r
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        let p = &mut self.pixels;
        let r: &mut [Color] = &mut p[start..start + self.width];
        r
    }
}

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a Color;
    type IntoIter = CanvasIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CanvasIterator {
            pixels: &self.pixels,
            current_idx: 0 as usize,
        }
    }
}

pub struct CanvasIterator<'a> {
    pixels: &'a Vec<Color>,
    current_idx: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
    type Item = &'a Color;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.pixels.len() {
            return None;
        }
        let result = Some(&self.pixels[self.current_idx]);
        self.current_idx += 1;
        result
    }
}
