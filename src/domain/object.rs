use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Point, RayTuple, Vector};
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(PartialEq, Debug, Clone)]
pub struct Shape {
    pub transformation: Matrix,
    pub material: Material,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
    pub shape: Shape,
    pub origin: Point,
    //radius: f64,
}

pub trait Renderable {
    fn local_intersect(&self, ray: &Ray) -> Intersections;
    fn local_normal_at(&self, point: &Point) -> Vector;
    fn shape(&self) -> &Shape;
    fn shape_mut(&mut self) -> &mut Shape;
    fn as_any(&self) -> &dyn Any;

    // Finds intersections of ray against sphere instance
    fn intersect(&self, ray: &Ray) -> Intersections {
        let inv_sphere_transform = self.shape().transformation.inverse();
        if inv_sphere_transform.is_none() {
            panic!("Unexpected non-invertible matrix.");
        }
        let localized_ray = ray.transform(&inv_sphere_transform.unwrap());

        self.local_intersect(&localized_ray)
    }

    // Computes the normal at given point.
    fn normal_at(&self, point: &Point) -> Vector {
        let mut st_inv = self.shape().transformation.inverse().unwrap();
        let local_point = &st_inv * point;
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = &*st_inv.transpose() * &local_normal;

        world_normal.normalize()
    }
}

impl PartialEq<Sphere> for dyn Renderable {
    fn eq(&self, other: &Sphere) -> bool {
        let this = &*(self.as_any()); // &* removes smart pointer container (Box, Arc, etc.)
        let that = &*(other.as_any());
        if this.type_id() == that.type_id() {
            let a = this
                .downcast_ref::<Sphere>()
                .expect("Unable to cast to Sphere!");
            let b = this
                .downcast_ref::<Sphere>()
                .expect("Unable to cast to Sphere!");
            a == b
        } else {
            false
        }
    }
}

impl PartialEq for dyn Renderable {
    fn eq(&self, other: &dyn Renderable) -> bool {
        let this = &*(self.as_any()); // &* removes smart pointer container (Box, Arc, etc.)
        let that = &*(other.as_any());

        let result;
        if this.type_id() == that.type_id() && that.is::<Sphere>() {
            let sphere = this
                .downcast_ref::<Sphere>()
                .expect("Unable to cast to Sphere!");
            result = &self == &sphere
        } else {
            result = false;
        }
        result
    }
}

// see https://users.rust-lang.org/t/derive-debug-not-playing-well-with-dyn/52398/2
impl Debug for dyn Renderable + '_ {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "shape: {:?} - {:?}",
            &self.shape().transformation,
            &self.shape().material
        )
    }
}

// Unit measure for shapes.
//const UNIT: f64 = 1.0;

impl Shape {
    // default constructor
    pub fn new_unit() -> Shape {
        Shape::new(crate::domain::matrix::IDENTITY.clone(), Material::new())
    }

    // parameter constructor
    pub fn new(transformation: Matrix, material: Material) -> Shape {
        Shape {
            transformation,
            material,
        }
    }
}

impl Sphere {
    const ORIGIN: Point = Point {
        ray_tuple: RayTuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
    };

    // constructor w/ no transformation matrix (identify matrix default)
    pub fn new_unit() -> Sphere {
        Sphere {
            origin: Sphere::ORIGIN,
            //radius: UNIT,
            shape: Shape::new_unit(),
        }
    }

    // constructor w/ initial transformation matrix
    pub fn new(transformation: Matrix) -> Sphere {
        Sphere {
            shape: Shape {
                transformation,
                ..Shape::new_unit()
            },
            ..Sphere::new_unit()
        }
    }

    // constructor w/ material argument
    pub fn new_material(material: Material) -> Sphere {
        Sphere {
            shape: Shape {
                material,
                ..Shape::new_unit()
            },
            ..Sphere::new_unit()
        }
    }
}

impl Renderable for Sphere {
    fn shape(&self) -> &Shape {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }

    // Finds intersections of ray against sphere instance
    fn local_intersect(&self, localized_ray: &Ray) -> Intersections {
        let sphere_to_ray = &localized_ray.origin - &self.origin;
        let a: f64 = localized_ray
            .direction
            .dot_product(&localized_ray.direction);
        let b: f64 = 2.0 * localized_ray.direction.dot_product(&sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
        let discriminant: f64 = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let mut ints = Intersections::new();
            ints.push(Intersection::new(t1, self))
                .push(Intersection::new(t2, self));
            ints
        }
    }

    // Computes the normal at given point.
    fn local_normal_at(&self, point: &Point) -> Vector {
        point - &Sphere::ORIGIN
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
