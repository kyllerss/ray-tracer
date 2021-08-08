use crate::domain::*;
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {

    // constructor
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color{red, green, blue}
    }
}

impl PartialEq for Color {

    fn eq(&self, other: &Self) -> bool {
        epsilon_eq(self.red, other.red)
            && epsilon_eq(self.green, other.green)
            && epsilon_eq(self.blue, other.blue)
    }
}

impl Add for Color {

    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.red + rhs.red;
        let g = self.green + rhs.green;
        let b = self.blue + rhs.blue;
        Color::new(r, g, b)
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.red - rhs.red;
        let g = self.green - rhs.green;
        let b = self.blue - rhs.blue;
        Color::new(r, g, b)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        let r = self.red * rhs.red;
        let g = self.green * rhs.green;
        let b = self.blue * rhs.blue;
        Color::new(r,g,b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        let r = self.red * rhs;
        let g = self.green * rhs;
        let b = self.blue * rhs;
        Color::new(r,g,b)
    }
}
