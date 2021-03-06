use crate::domain::*;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    // constructor
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::BLACK
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        epsilon_eq(self.red, other.red)
            && epsilon_eq(self.green, other.green)
            && epsilon_eq(self.blue, other.blue)
    }
}

impl<'a> Add for &'a Color {
    type Output = Color;
    fn add(self: &'a Color, rhs: &'a Color) -> Self::Output {
        let r = self.red + rhs.red;
        let g = self.green + rhs.green;
        let b = self.blue + rhs.blue;
        Color::new(r, g, b)
    }
}

impl<'a> Sub for &'a Color {
    type Output = Color;
    fn sub(self: &'a Color, rhs: &'a Color) -> Self::Output {
        let r = self.red - rhs.red;
        let g = self.green - rhs.green;
        let b = self.blue - rhs.blue;
        Color::new(r, g, b)
    }
}

impl<'a> Mul<&'a Color> for &'a Color {
    type Output = Color;
    fn mul(self: &'a Color, rhs: &'a Color) -> Self::Output {
        let r = self.red * rhs.red;
        let g = self.green * rhs.green;
        let b = self.blue * rhs.blue;
        Color::new(r, g, b)
    }
}

impl<'a> Mul<f32> for &'a Color {
    type Output = Color;
    fn mul(self: &'a Color, rhs: f32) -> Self::Output {
        let r = self.red * rhs;
        let g = self.green * rhs;
        let b = self.blue * rhs;
        Color::new(r, g, b)
    }
}
