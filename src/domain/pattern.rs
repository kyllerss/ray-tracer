use crate::domain::color::Color;
use crate::domain::Point;

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    STRIPED { a: Color, b: Color },
}

impl Pattern {
    // constructor
    pub fn new_striped(a: Color, b: Color) -> Pattern {
        Pattern::STRIPED { a, b }
    }

    // calculates color for the given pattern at the given point
    pub fn color_at<'a>(&'a self, point: &'a Point) -> &'a Color {
        match &self {
            Pattern::STRIPED { a, b } => color_at_striped(a, b, point),
        }
    }
}

// striped colors alternating across x axis
fn color_at_striped<'a>(a: &'a Color, b: &'a Color, point: &'a Point) -> &'a Color {
    let result;
    if point.x().floor() % 2.0 == 0.0 {
        result = a;
    } else {
        result = b;
    }
    result
}
