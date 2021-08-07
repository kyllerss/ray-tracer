mod operations;

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

    // calculates magnitude
    pub fn magnitude(&self) -> f64 {
        let sum = self.x.powi(2)
                  + self.y.powi(2)
                  + self.z.powi(2);
        f64::sqrt(sum)
    }

    // normalize vector
    pub fn normalize(&self) -> RayTuple {
        let x_norm = self.x / self.magnitude();
        let y_norm = self.y / self.magnitude();
        let z_norm = self.z / self.magnitude();
        let w_norm = self.w / self.magnitude();
        RayTuple::new(x_norm, y_norm, z_norm, w_norm)
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

// Convert into a raytracing tuple
pub trait ToRayTuple {
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

impl Vector {
    const W: f64 = 0.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { ray_tuple: RayTuple::new(x, y, z, Vector::W) }
    }

    // calculates magnitude
    pub fn magnitude(&self) -> f64 {
        self.ray_tuple.magnitude()
    }

    // normalize vector
    pub fn normalize(&self) -> Vector {
        let rt_norm = self.ray_tuple.normalize();
        Vector::new(rt_norm.x, rt_norm.y, rt_norm.z)
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
