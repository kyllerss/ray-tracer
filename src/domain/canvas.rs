use std::ops::{Index, IndexMut};

use crate::domain::color::Color;

#[derive(Clone, Debug)]
pub struct Canvas<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<&'a Color>,
}

impl<'a> Canvas<'a> {
    // constructor
    pub fn new(width: usize, height: usize, default_color: &Color) -> Canvas {
        let vec_length = width * height;
        let pixels = vec![default_color; vec_length];

        Canvas {
            width,
            height,
            pixels,
        }
    }
}

impl<'a> Index<usize> for Canvas<'a> {
    type Output = [&'a Color];
    fn index(&self, x: usize) -> &[&'a Color] {
        let start = x * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<'a> IndexMut<usize> for Canvas<'a> {
    fn index_mut(&mut self, x: usize) -> &mut [&'a Color] {
        let start = x * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

impl<'a> IntoIterator for &Canvas<'a> {
    type Item = &'a Color;
    type IntoIter = CanvasIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CanvasIterator::new(&mut self.pixels.into_iter())
    }
}

struct CanvasIterator<'a> {
    pixel_itr: &'a mut dyn Iterator<Item = &'a Color>,
}

impl<'a> CanvasIterator<'a> {
    fn new(itr: &'a mut dyn Iterator<Item = &'a Color>) -> CanvasIterator<'a> {
        CanvasIterator{pixel_itr: itr}
    }
}

impl<'a> Iterator for CanvasIterator<'a> {
    type Item = &'a Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.pixel_itr.next()
    }
}
