use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    contents: Vec<f64>,
}

impl Matrix {
    // constructor
    pub fn new(width: usize, height: usize, contents: Vec<f64>) -> Matrix {
        if width * height != contents.len() {
            panic!("Dimensions for matrix do not match contents.");
        }

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

fn m(row: usize, col: usize, a: &Matrix, b: &Matrix) -> f64 {
    a[row][0] * b[0][col] + a[row][1] * b[1][col] + a[row][2] * b[2][col] + a[row][3] * b[3][col]
}

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'a Matrix) -> Self::Output {
        use crate::matrix;
        matrix![m(0,0,&self,rhs), m(0,1,&self,rhs), m(0,2,&self,rhs), m(0,3,&self,rhs);
                m(1,0,&self,rhs), m(1,1,&self,rhs), m(1,2,&self,rhs), m(1,3,&self,rhs);
                m(2,0,&self,rhs), m(2,1,&self,rhs), m(2,2,&self,rhs), m(2,3,&self,rhs);
                m(3,0,&self,rhs), m(3,1,&self,rhs), m(3,2,&self,rhs), m(3,3,&self,rhs)]
    }
}
