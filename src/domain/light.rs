use crate::domain::color::Color;
use crate::domain::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    // constructor
    pub fn new(position: Point, intensity: Color) -> Light {
        Light {
            position,
            intensity,
        }
    }
}
