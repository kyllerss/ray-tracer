use crate::domain::color::Color;
use crate::domain::matrix::Matrix;
use crate::domain::object::Object;
use crate::domain::Point;

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    STRIPED {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
}

impl Pattern {
    // constructor
    pub fn new_striped(a: Color, b: Color) -> Pattern {
        Pattern::new_striped_with_transformation(a, b, crate::domain::matrix::IDENTITY.clone())
    }

    // constructor
    pub fn new_striped_with_transformation(a: Color, b: Color, transformation: Matrix) -> Pattern {
        Pattern::STRIPED {
            a,
            b,
            transformation,
        }
    }

    // TODO Turn this into a trait method to obviate pattern matching
    pub fn transformation(&self) -> &Matrix {
        match self {
            Pattern::STRIPED { transformation, .. } => transformation,
        }
    }

    // calculates color for the given pattern at the given point
    pub fn color_at(&self, object: &Object, world_point: &Point) -> &Color {
        // convert pattern to object and pattern orientations
        let object_point = &object.shape().transformation.inverse().unwrap() * world_point;
        let pattern_point = &self.transformation().inverse().unwrap() * &object_point;

        match &self {
            Pattern::STRIPED { a, b, .. } => color_at_striped(a, b, &pattern_point),
        }
    }
}

// striped colors alternating across x axis
fn color_at_striped<'a, 'b>(a: &'a Color, b: &'a Color, point: &'b Point) -> &'a Color {
    if point.x().floor() % 2.0 == 0.0 {
        a
    } else {
        b
    }
}
