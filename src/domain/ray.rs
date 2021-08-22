use crate::domain::{Point, Vector};

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
    pub fn position(&self, distance: f32) -> Point {
        self.origin + self.direction * distance as f64
    }
}
