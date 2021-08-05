mod tests;

#[derive(Copy, Clone, Debug)]
struct RayTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl RayTuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> RayTuple {
        RayTuple{x, y, z, w}
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

impl ToRayTuple for Point {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64) {
        (self.ray_tuple.x,
         self.ray_tuple.y,
         self.ray_tuple.z,
         self.ray_tuple.w)
    }
}

impl Vector {
    const W: f64 = 0.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { ray_tuple: RayTuple::new(x, y, z, Vector::W) }
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