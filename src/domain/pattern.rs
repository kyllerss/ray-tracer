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

    // calculates color for the given pattern at the given point
    pub fn color_at<'a>(&'a self, object: &'a Object, point: &'a Point) -> &'a Color {
        match &self {
            Pattern::STRIPED {
                a,
                b,
                transformation,
            } => color_at_striped(a, b, object, point, transformation),
        }
    }
}

// striped colors alternating across x axis
fn color_at_striped<'a>(
    a: &'a Color,
    b: &'a Color,
    object: &'a Object,
    world_point: &'a Point,
    transformation: &'a Matrix,
) -> &'a Color {
    let object_point = &object.shape().transformation.inverse().unwrap() * world_point;
    let pattern_point = &transformation.inverse().unwrap() * &object_point;

    let result;
    if pattern_point.x().floor() % 2.0 == 0.0 {
        result = a;
    } else {
        result = b;
    }
    result
}
