use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) mod camera;
pub(crate) mod canvas;
pub(crate) mod color;
pub(crate) mod intersection;
pub(crate) mod light;
pub(crate) mod material;
pub(crate) mod matrix;
pub(crate) mod object;
pub(crate) mod operations;
pub(crate) mod pattern;
pub(crate) mod ray;
pub(crate) mod world;

use num::{Float, NumCast};

pub const EPSILON: f64 = 0.0001;
pub fn epsilon_eq<F>(a: F, b: F) -> bool
where
    F: Float + NumCast,
{
    let abs: F = (a - b).abs();
    let c: Option<f64> = num::cast(abs);
    c.unwrap() < EPSILON
}

// Prints the type of the variable
pub fn type_id<'a, T: std::any::Any>(_: T) -> String {
    format!("{:?}", std::any::TypeId::of::<T>())
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Id {
    id: usize,
}

impl Id {
    // Generates a unique global id for given run.
    // stolen from: https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805/5
    pub fn new() -> Id {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Id {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct RayTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl RayTuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> RayTuple {
        RayTuple { x, y, z, w }
    }

    // calculates magnitude
    pub fn magnitude(&self) -> f64 {
        let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        f64::sqrt(sum)
    }

    // normalize vector
    pub fn normalize(&self) -> RayTuple {
        let magnitude = self.magnitude();
        let x_norm = self.x / magnitude;
        let y_norm = self.y / magnitude;
        let z_norm = self.z / magnitude;
        let w_norm = self.w / magnitude;
        RayTuple::new(x_norm, y_norm, z_norm, w_norm)
    }

    // setter x
    pub fn set_x(self, x_val: f64) -> Self {
        RayTuple { x: x_val, ..self }
    }

    // setter y
    pub fn set_y(self, y_val: f64) -> Self {
        RayTuple { y: y_val, ..self }
    }

    // setter z
    pub fn set_z(self, z_val: f64) -> Self {
        RayTuple { z: z_val, ..self }
    }
}

impl PartialEq for RayTuple {
    fn eq(&self, other: &Self) -> bool {
        epsilon_eq(self.x, other.x)
            && epsilon_eq(self.y, other.y)
            && epsilon_eq(self.z, other.z)
            && epsilon_eq(self.w, other.w)
    }
}

// Convert into a raytracing tuple
pub trait ToRayTuple {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64);
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    ray_tuple: RayTuple,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    ray_tuple: RayTuple,
}

impl Point {
    const W: f64 = 1.0;

    pub const ORIGIN: Point = Point {
        ray_tuple: RayTuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
    };

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            ray_tuple: RayTuple::new(x, y, z, Point::W),
        }
    }

    // accessor
    pub fn x(&self) -> f64 {
        self.ray_tuple.x
    }

    // accessor
    pub fn y(&self) -> f64 {
        self.ray_tuple.y
    }

    // accessor
    pub fn z(&self) -> f64 {
        self.ray_tuple.z
    }

    pub fn set_x(&self, x: f64) -> Self {
        Self {
            ray_tuple: self.ray_tuple.set_x(x),
        }
    }

    pub fn set_y(&self, y: f64) -> Self {
        Self {
            ray_tuple: self.ray_tuple.set_y(y),
        }
    }

    pub fn set_z(&self, z: f64) -> Self {
        Self {
            ray_tuple: self.ray_tuple.set_z(z),
        }
    }

    pub fn add_x(&self, val: f64) -> Self {
        let orig = self.x();
        self.set_x(orig + val)
    }

    pub fn add_y(&self, val: f64) -> Self {
        let orig = self.y();
        self.set_y(orig + val)
    }

    pub fn add_z(&self, val: f64) -> Self {
        let orig = self.z();
        self.set_z(orig + val)
    }

    pub fn mult_x(&self, val: f64) -> Self {
        let orig = self.x();
        self.set_x(orig * val)
    }

    pub fn mult_y(&self, val: f64) -> Self {
        let orig = self.y();
        self.set_y(orig * val)
    }

    pub fn mult_z(&self, val: f64) -> Self {
        let orig = self.z();
        self.set_z(orig * val)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.ray_tuple == other.ray_tuple
    }
}

impl ToRayTuple for Point {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64) {
        (
            self.ray_tuple.x,
            self.ray_tuple.y,
            self.ray_tuple.z,
            self.ray_tuple.w,
        )
    }
}

impl Vector {
    const W: f64 = 0.0;

    // constructor
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            ray_tuple: RayTuple::new(x, y, z, Vector::W),
        }
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

    // calculates the dot product
    pub fn dot_product(&self, v: &Vector) -> f64 {
        self.ray_tuple.x * v.ray_tuple.x
            + self.ray_tuple.y * v.ray_tuple.y
            + self.ray_tuple.z * v.ray_tuple.z
    }

    // calculates cross product
    pub fn cross_product(&self, v: &Vector) -> Vector {
        let rt1 = self.ray_tuple;
        let rt2 = v.ray_tuple;
        let x = rt1.y * rt2.z - rt1.z * rt2.y;
        let y = rt1.z * rt2.x - rt1.x * rt2.z;
        let z = rt1.x * rt2.y - rt1.y * rt2.x;
        Vector::new(x, y, z)
    }

    // accessor
    pub fn x(&self) -> f64 {
        self.ray_tuple.x
    }

    // accessor
    pub fn y(&self) -> f64 {
        self.ray_tuple.y
    }

    // accessor
    pub fn z(&self) -> f64 {
        self.ray_tuple.z
    }

    // Reflects vector around a given normal
    pub fn reflect(self: &Vector, normal: &Vector) -> Vector {
        self - &(&(normal * 2_f64) * self.dot_product(normal))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.ray_tuple == other.ray_tuple
    }
}

impl ToRayTuple for Vector {
    fn to_ray_tuple(&self) -> (f64, f64, f64, f64) {
        (
            self.ray_tuple.x,
            self.ray_tuple.y,
            self.ray_tuple.z,
            self.ray_tuple.w,
        )
    }
}
