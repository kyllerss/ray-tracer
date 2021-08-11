//use std::ops::{Index, IndexMut};
use std::ops::{Index, IndexMut};

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
        let pixels = vec![default_color; vec_length];

        Canvas {
            width,
            height,
            pixels,
        }
    }
}

impl Index<usize> for Canvas {
    type Output = [Color];
    fn index(&self, x: usize) -> &Self::Output {
        let start = x * self.height;
        let p = &self.pixels;
        let r: &[Color] = &p[start..start + self.height];
        r
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.height;
        let p = &mut self.pixels;
        let r: &mut [Color] = &mut p[start..start + self.height];
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
