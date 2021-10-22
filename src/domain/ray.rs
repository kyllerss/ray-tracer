use crate::domain::matrix::Matrix;
use crate::domain::{Point, Vector};

#[derive(PartialEq, Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    // constructor
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    // calculates points at given position
    pub fn position(&self, distance: f64) -> Point {
        &self.origin + &(&self.direction * distance)
    }

    // transforms ray (building new instance)
    pub fn transform(&self, matrix: &Matrix) -> Ray {
        let new_origin = matrix * &self.origin;
        let new_direction = matrix * &self.direction;

        Ray::new(new_origin, new_direction)
    }
}
