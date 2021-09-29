use crate::domain::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<'a> Add for &'a RayTuple {
    type Output = RayTuple;
    fn add(self: &'a RayTuple, rhs: &'a RayTuple) -> Self::Output {
        RayTuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<'a> Sub for &'a RayTuple {
    type Output = RayTuple;
    fn sub(self: &'a RayTuple, rhs: &'a RayTuple) -> Self::Output {
        RayTuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for RayTuple {
    type Output = RayTuple;
    fn neg(self) -> Self::Output {
        RayTuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<'a> Mul<f64> for &'a RayTuple {
    type Output = RayTuple;
    fn mul(self: &'a RayTuple, rhs: f64) -> Self::Output {
        RayTuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl<'a> Div<f64> for &'a RayTuple {
    type Output = RayTuple;
    fn div(self: &'a RayTuple, rhs: f64) -> Self::Output {
        RayTuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl<'a> Add<&'a Vector> for &'a Point {
    type Output = Point;
    fn add(self: &'a Point, rhs: &'a Vector) -> Self::Output {
        let result = &self.ray_tuple + &rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl<'a> Add<&'a Point> for &'a Point {
    type Output = Vector;
    fn add(self: &'a Point, rhs: &'a Point) -> Self::Output {
        let result = &self.ray_tuple + &rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl<'a> Sub<&'a Point> for &'a Point {
    type Output = Vector;
    fn sub(self: &'a Point, rhs: &'a Point) -> Self::Output {
        let result = &self.ray_tuple - &rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl<'a> Add<&'a Point> for &'a Vector {
    type Output = Point;
    fn add(self: &'a Vector, rhs: &'a Point) -> Self::Output {
        let result = &self.ray_tuple + &rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl<'a> Sub<&'a Vector> for &'a Point {
    type Output = Point;
    fn sub(self: &'a Point, rhs: &'a Vector) -> Self::Output {
        let result = &self.ray_tuple - &rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl<'a> Add<&'a Vector> for &'a Vector {
    type Output = Vector;
    fn add(self: &'a Vector, rhs: &'a Vector) -> Self::Output {
        let result = &self.ray_tuple + &rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl<'a> Sub<&'a Vector> for &'a Vector {
    type Output = Vector;
    fn sub(self: &'a Vector, rhs: &'a Vector) -> Self::Output {
        let result = &self.ray_tuple - &rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        let n = -self.ray_tuple;
        Vector::new(n.x, n.y, n.z)
    }
}

impl<'a> Mul<f64> for &'a Vector {
    type Output = Vector;
    fn mul(self: &'a Vector, rhs: f64) -> Self::Output {
        let rt = &self.ray_tuple * rhs;
        Vector::new(rt.x, rt.y, rt.z)
    }
}

impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;
    fn div(self: &'a Vector, rhs: f64) -> Self::Output {
        let rt = &self.ray_tuple / rhs;
        Vector::new(rt.x, rt.y, rt.z)
    }
}
