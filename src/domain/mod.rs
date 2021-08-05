mod tests;

// Convert into a raytracing tuple
trait RayTuple {
    fn to_tuple(&self) -> (f64, f64, f64, f64);
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    const W: f64 = 1.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {x, y, z}
    }
}

impl RayTuple for Point {
    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, Point::W)
    }
}

impl Vector {
    const W: f64 = 0.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {x, y, z}
    }
}

impl RayTuple for Vector {
    fn to_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, Vector::W)
    }
}