use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::domain::*;

impl Add for RayTuple {

    type Output = RayTuple;
    fn add(self, rhs: Self) -> Self::Output {
        RayTuple::new(self.x + rhs.x,
                      self.y + rhs.y,
                      self.z + rhs.z,
                      self.w + rhs.w)
    }
}

impl Sub for RayTuple {

    type Output = RayTuple;
    fn sub(self, rhs: Self) -> Self::Output {
        RayTuple::new(self.x - rhs.x,
                      self.y - rhs.y,
                      self.z - rhs.z,
                      self.w - rhs.w)
    }
}

impl Neg for RayTuple {
    type Output = RayTuple;
    fn neg(self) -> Self::Output {
        RayTuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for RayTuple {
    type Output = RayTuple;
    fn mul(self, rhs: f64) -> Self::Output {
        RayTuple::new(self.x * rhs,
                      self.y * rhs,
                      self.z * rhs,
                      self.w * rhs)
    }
}

impl Div<f64> for RayTuple {
    type Output = RayTuple;
    fn div(self, rhs: f64) -> Self::Output {
        RayTuple::new(self.x / rhs,
                      self.y / rhs,
                      self.z / rhs,
                      self.w / rhs)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        let result = self.ray_tuple + rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl Add<Point> for Point {
    type Output = Vector;
    fn add(self, rhs: Point) -> Self::Output {
        let result = self.ray_tuple + rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        let result = self.ray_tuple - rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        let result = self.ray_tuple + rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        let result = self.ray_tuple - rhs.ray_tuple;
        Point::new(result.x, result.y, result.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        let result = self.ray_tuple + rhs.ray_tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        let result = self.ray_tuple - rhs.ray_tuple;
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

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let rt = self.ray_tuple * rhs;
        Vector::new(rt.x, rt.y, rt.z)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        let rt = self.ray_tuple / rhs;
        Vector::new(rt.x, rt.y, rt.z)
    }
}