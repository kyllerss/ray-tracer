use crate::domain::{Point, Vector};
use lazy_static::lazy_static;
use num::Integer;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug)]
struct CachedComponents {
    inv_contents: Option<Vec<f64>>,
    determinant: f64,
}

#[derive(Clone, Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    contents: Vec<f64>,
    cache: Option<CachedComponents>,
}

// All references need to be dereferenced
#[rustfmt::skip::macros(vec)]
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

#[rustfmt::skip::macros(vec, matrix)]
impl Matrix {
    // constructor (no caching)
    pub fn new_uncached(width: usize, height: usize, contents: Vec<f64>) -> Matrix {
        if width * height != contents.len() {
            panic!("Dimensions for matrix do not match contents.");
        }

        Matrix {
            width,
            height,
            contents,
            cache: Option::None,
        }
    }

    // constructor
    pub fn new(width: usize, height: usize, contents: Vec<f64>) -> Matrix {
        let mut matrix = Matrix::new_uncached(width, height, contents);
        matrix.initialize_cache();
        matrix
    }

    fn initialize_cache(&mut self) {
        let inv_matrix = self.inverse();

        let mut cache = CachedComponents {
            inv_contents: inv_matrix.map_or(Option::None, |m| Option::Some(m.contents)),
            determinant: self.determinant(),
        };

        self.cache = Option::Some(cache);
    }

    // new translation matrix
    pub fn new_translation(x: f64, y: f64, z: f64) -> Matrix {
        let contents: Vec<f64> = vec![1.0, 0.0, 0.0,  x ,
                                      0.0, 1.0, 0.0,  y ,
                                      0.0, 0.0, 1.0,  z ,
                                      0.0, 0.0, 0.0, 1.0];
        Matrix::new(4, 4, contents)
    }

    // new translation matrix
    pub fn new_scaling(x: f64, y: f64, z: f64) -> Matrix {
        let contents: Vec<f64> = vec![ x , 0.0, 0.0, 0.0,
                                      0.0,  y , 0.0, 0.0,
                                      0.0, 0.0,  z , 0.0,
                                      0.0, 0.0, 0.0, 1.0];
        Matrix::new(4, 4, contents)
    }

    // rotation along x-axis
    pub fn new_rotation_x(radians: f64) -> Matrix {
        let contents: Vec<f64> = vec![ 1.0, 0.0,           0.0,            0.0,
                                       0.0, radians.cos(), -radians.sin(), 0.0,
                                       0.0, radians.sin(),  radians.cos(), 0.0,
                                       0.0, 0.0,            0.0,           1.0];
        Matrix::new(4, 4, contents)
    }

    // rotation along y-axis
    pub fn new_rotation_y(radians: f64) -> Matrix {
        let contents: Vec<f64> = vec![ radians.cos(),  0.0,  radians.sin(),  0.0,
                                       0.0,            1.0,  0.0,            0.0,
                                       -radians.sin(), 0.0,  radians.cos(),  0.0,
                                       0.0,            0.0,  0.0,            1.0];
        Matrix::new(4, 4, contents)
    }

    // rotation along z-axis
    pub fn new_rotation_z(radians: f64) -> Matrix {
        let contents: Vec<f64> = vec![ radians.cos(),  -radians.sin(),  0.0, 0.0,
                                       radians.sin(),  radians.cos(),   0.0, 0.0,
                                       0.0,            0.0,             1.0, 0.0,
                                       0.0,            0.0,             0.0, 1.0];
        Matrix::new(4, 4, contents)
    }

    // shearing
    pub fn new_shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let contents: Vec<f64> = vec![ 1.0, x_y, x_z, 0.0,
                                       y_x, 1.0, y_z, 0.0,
                                       z_x, z_y, 1.0, 0.0,
                                       0.0, 0.0, 0.0, 1.0];
        Matrix::new(4, 4, contents)
    }

    // view transformation
    pub fn new_view_transformation(from: &Point, to: &Point, up: &Vector) -> Matrix {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross_product(&upn);
        let true_up = left.cross_product(&forward);

        let contents: Vec<f64> = vec![ left.x()    , left.y()    , left.z()    , 0.0,
                                       true_up.x() , true_up.y() , true_up.z() , 0.0,
                                       -forward.x(), -forward.y(), -forward.z(), 0.0,
                                       0.0         , 0.0         , 0.0         , 1.0];
        let orientation = Matrix::new(4, 4, contents);
        &orientation * &Matrix::new_translation(-from.x(), -from.y(), -from.z())
    }

    // transposes a matrix
    pub fn transpose(&mut self) -> &mut Self {
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

        self.initialize_cache();

        self
    }

    // calculate determinant on 2x2 matrix
    // Returns None if matrix is not 2x2
    pub fn determinant(&self) -> f64 {
        if self.cache.is_some() {
            return self.cache.as_ref().unwrap().determinant;
        }

        let mut result: f64 = 0.0;
        if self.width == 2 && self.height == 2 {
            result = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        } else {
            for col in 0..self.width {
                result += self[0][col] * self.cofactor(0, col);
            }
        }
        result
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

        Matrix::new_uncached(self.width - 1, self.height - 1, v)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sm = &self.submatrix(row, col);
        sm.determinant()
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

    pub fn is_invertible(&self) -> bool {
        self.cache
            .as_ref()
            .map_or(self.determinant(), |c| c.determinant)
            != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if !self.is_invertible() {
            return None;
        }

        if self.cache.is_some() {
            let inv_c = self.cache.as_ref().unwrap().inv_contents.clone().unwrap();
            let inv_m = Matrix::new_uncached(self.width, self.height, inv_c);
            return Option::Some(inv_m);
        }

        let mut m_inv = Matrix::new_uncached(
            self.width,
            self.height,
            vec![f64::default(); self.width * self.height],
        );

        let determinant = self.determinant();

        for row in 0..self.height {
            for col in 0..self.width {
                let cofactor = self.cofactor(row, col);
                m_inv[col][row] = cofactor / determinant; // col/row switch handles transpose
            }
        }

        Some(m_inv)
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.contents.len() == other.contents.len()
            && self
                .contents
                .iter()
                .zip(other.contents.iter())
                .all(|(a, b): (&f64, &f64)| crate::domain::epsilon_eq(*a, *b))
    }
}
