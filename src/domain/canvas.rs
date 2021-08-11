//use std::ops::{Index, IndexMut};
use std::ops::Index;

use crate::domain::color::Color;
//use std::iter::{StepBy, Chain, Zip, Intersperse, IntersperseWith, Map, Filter, FilterMap, Enumerate, Peekable, SkipWhile, TakeWhile, MapWhile, Skip, Take, Scan, FlatMap, Flatten, Fuse, Inspect, FromIterator, Rev, Copied, Cloned, Cycle, Sum, Product, TrustedRandomAccess};
//use std::convert::Infallible;
//use std::cmp::Ordering;

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
        let mut pixels = vec![default_color; vec_length];
        // pixels.push(default_color);

        Canvas {
            width,
            height,
            pixels,
        }
    }
}

impl<'a> Index<usize> for &'a Canvas {
    type Output = [Color];
    fn index(&self, x: usize) -> &Self::Output {
        let start = x * self.width;
        let p = &self.pixels;
        let r: &[Color] = &p[start..start + self.width];
        r
    }
}



// impl<'a> IndexMut<usize> for &'a Canvas {
//     fn index_mut(&mut self, x: usize) -> &mut [Color] {
//         let start = x * self.width;
//         //let p = &mut self.pixels;
//         let mut p = self.pixels;
//         &mut p[start..start + self.width]
//     }
// }

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a Color;
    type IntoIter = CanvasIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CanvasIterator{pixels: &self.pixels, current_idx: 0 as usize}
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