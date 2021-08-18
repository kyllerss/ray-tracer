use crate::domain::{Point, Vector};
use lazy_static::lazy_static;
use num::Integer;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    contents: Vec<f64>,
}

// All references need to be dereferenced
#[rustfmt::skip::macros(vec, matrix)]
lazy_static! {
    pub static ref IDENTITY: Matrix = Matrix::new(
        4,
        4,
        vec![1.0, 0.0, 0.0, 0.0,
             0.0, 1.0, 0.0, 0.0,
             0.0, 0.0, 1.0, 0.0,
             0.0, 0.0, 0.0, 1.0],
    );
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

    // transposes a matrix
    pub fn transpose(&mut self) {
        'outer: for row in 0..self.height {
            for col in 0..self.width {
                if row == col {
                    continue 'outer; // iterate only up to diagonal
                }

                let a = row * self.height + col;
                let b = col * self.width + row;

                self.contents.swap(a, b);
            }
        }
    }

    // calculate determinant on 2x2 matrix
    // Returns None if matrix is not 2x2
    pub fn determinant(&self) -> Option<f64> {
        if self.width != 2 || self.height != 2 {
            return None;
        }

        let result = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        Some(result)
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut v: Vec<f64> = Vec::with_capacity(row * col);
        for row_idx in 0..self.height {
            if row_idx == row {
                continue; // exclude row
            }

            for col_idx in 0..self.width {
                if col_idx == col {
                    continue; // exclude col
                }

                v.push(self[row_idx][col_idx]);
            }
        }

        Matrix::new(self.width - 1, self.height - 1, v)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sm = &self.submatrix(row, col);
        sm.determinant().unwrap()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        let negate = (row + col).is_odd();
        if negate {
            -minor
        } else {
            minor
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

impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'a Matrix) -> Self::Output {
        use crate::matrix;

        //unrolled loop
        matrix![m(0,0,&self,rhs), m(0,1,&self,rhs), m(0,2,&self,rhs), m(0,3,&self,rhs);
                m(1,0,&self,rhs), m(1,1,&self,rhs), m(1,2,&self,rhs), m(1,3,&self,rhs);
                m(2,0,&self,rhs), m(2,1,&self,rhs), m(2,2,&self,rhs), m(2,3,&self,rhs);
                m(3,0,&self,rhs), m(3,1,&self,rhs), m(3,2,&self,rhs), m(3,3,&self,rhs)]
    }
}

impl<'a> Mul<&'a Point> for &'a Matrix {
    type Output = Point;
    fn mul(self, rhs: &'a Point) -> Self::Output {
        let x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * Point::W;
        let y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * Point::W;
        let z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * Point::W;
        Point::new(x, y, z)
    }
}

impl<'a> Mul<&'a Vector> for &'a Matrix {
    type Output = Vector;
    fn mul(self, rhs: &'a Vector) -> Self::Output {
        let x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * Vector::W;
        let y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * Vector::W;
        let z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * Vector::W;
        Vector::new(x, y, z)
    }
}
