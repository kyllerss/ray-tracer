use std::ops::Add;

mod tests;

#[derive(Copy, Clone, Debug)]
struct RayTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl RayTuple {
    const EPSILON: f64 = 0.00001;

    fn new(x: f64, y: f64, z: f64, w: f64) -> RayTuple {
        RayTuple{x, y, z, w}
    }

    fn epsilon_eq(a: f64, b: f64) -> bool {
        f64::abs(a - b) < RayTuple::EPSILON
    }
}

impl PartialEq for RayTuple {

    fn eq(&self, other: &Self) -> bool {
        RayTuple::epsilon_eq(self.x, other.x)
            && RayTuple::epsilon_eq(self.y, other.y)
            && RayTuple::epsilon_eq(self.z, other.z)
            && RayTuple::epsilon_eq(self.w, other.w)
    }
}

impl Add for RayTuple {

    type Output = RayTuple;
    fn add(self, rhs: Self) -> Self::Output {
        RayTuple::new(self.x + rhs.x,
                      self.y + rhs.y,
                      self.z + rhs.z,
                      self.w + rhs.w)
    }
}

// Convert into a raytracing tuple
trait ToRayTuple {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64);
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    ray_tuple: RayTuple
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    ray_tuple: RayTuple
}

impl Point {
    const W: f64 = 1.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { ray_tuple: RayTuple::new(x, y, z, Point::W) }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.ray_tuple == other.ray_tuple
    }
}

impl ToRayTuple for Point {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64) {
        (self.ray_tuple.x,
         self.ray_tuple.y,
         self.ray_tuple.z,
         self.ray_tuple.w)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        let rt = self.ray_tuple;
        let rhs_rt = rhs.ray_tuple;
        let added = rt + rhs_rt;
        Point::new(added.x, added.y, added.z)
    }
}

impl Vector {
    const W: f64 = 0.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { ray_tuple: RayTuple::new(x, y, z, Vector::W) }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.ray_tuple == other.ray_tuple
    }
}

impl ToRayTuple for Vector {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64) {
        (self.ray_tuple.x,
         self.ray_tuple.y,
         self.ray_tuple.z,
         self.ray_tuple.w)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        let rt = self.ray_tuple;
        let rhs_rt = rhs.ray_tuple;
        let added = rt + rhs_rt;
        Point::new(added.x, added.y, added.z)
    }
}
