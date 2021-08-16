use std::ops::{Index, IndexMut};

pub struct Matrix {
    pub width: usize,
    pub height: usize,
    contents: Vec<f64>,
}

impl Matrix {
    // constructor
    pub fn new(width: usize, height: usize, contents: Vec<f64>) -> Matrix {
        Matrix {
            width,
            height,
            contents,
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        let p = &self.contents;
        let r: &[f64] = &p[start..start + self.width];
        r
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        let p = &mut self.contents;
        let r: &mut [f64] = &mut p[start..start + self.width];
        r
    }
}
