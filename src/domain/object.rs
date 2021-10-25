use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Point, RayTuple, Vector};
use std::fmt::Debug;

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

#[derive(PartialEq, Debug, Clone)]
pub struct Null {
    pub shape: Shape,
    // pub saved_ray: Option<Ray>, // for unit testing
}

#[derive(PartialEq, Debug, Clone)]
pub struct Plane {
    pub shape: Shape,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Null(Null), // throw-away test implementation
    Plane(Plane),
}

impl Object {
    // TODO Define trait that returns these, so that the match is not necessary.
    fn local_intersect(&self, ray: &Ray) -> Intersections {
        let ints = match self {
            Object::Sphere(sphere) => sphere.local_intersect(ray),
            Object::Null(null) => null.local_intersect(ray),
            Object::Plane(plane) => plane.local_intersect(ray),
        };
        let mut result = Intersections::new();
        ints.iter().for_each(|int| {
            result.push(Intersection::new(*int, self));
        });
        result
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    fn local_normal_at(&self, point: &Point) -> Vector {
        match self {
            Object::Sphere(sphere) => sphere.local_normal_at(point),
            Object::Null(null) => null.local_normal_at(point),
            Object::Plane(plane) => plane.local_normal_at(point),
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape(&self) -> &Shape {
        match self {
            Object::Sphere(sphere) => &sphere.shape,
            Object::Null(null) => &null.shape,
            Object::Plane(plane) => &plane.shape,
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape_mut(&mut self) -> &mut Shape {
        match self {
            Object::Sphere(sphere) => &mut sphere.shape,
            Object::Null(null) => &mut null.shape,
            Object::Plane(plane) => &mut plane.shape,
        }
    }

    // Finds intersections of ray against sphere instance
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let inv_sphere_transform = self.shape().transformation.inverse();
        if inv_sphere_transform.is_none() {
            panic!("Unexpected non-invertible matrix.");
        }
        let localized_ray = ray.transform(&inv_sphere_transform.unwrap());

        self.local_intersect(&localized_ray)
    }

    // Computes the normal at given point.
    pub fn normal_at(&self, point: &Point) -> Vector {
        let mut st_inv = self.shape().transformation.inverse().unwrap();
        let local_point = &st_inv * point;
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = &*st_inv.transpose() * &local_normal;

        world_normal.normalize()
    }

    // builders/constructors
    pub fn new_sphere_unit() -> Object {
        Object::Sphere(Sphere::new_unit())
    }

    pub fn new_sphere_with_matrix(matrix: Matrix) -> Object {
        Object::Sphere(Sphere::new(matrix))
    }

    pub fn new_sphere_with_material(material: Material) -> Object {
        Object::Sphere(Sphere::new_material(material))
    }

    // test item with minimal implementation
    pub fn new_null() -> Object {
        Object::Null(Null::new())
    }

    // pub fn new_plane() -> Object {
    //     Object::Plane(Plane::new())
    // }

    pub fn new_plane_with_transformation_and_material(t: Matrix, m: Material) -> Object {
        Object::Plane(Plane::new_with_transformation_and_material(t, m))
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

impl Null {
    pub fn new() -> Null {
        Null {
            shape: Shape::new_unit(),
            //saved_ray: Option::None,
        }
    }
    pub(crate) fn local_intersect(&self, _ray: &Ray) -> Vec<f64> {
        // NOTE: Ch9 - test 3 require mutability on an intersect calculation that otherwise
        // never leads to mutable state. This commented-out line below causes the API to lead
        // through to other callers - leading to additional complications with lifetimes and
        // the borrow-checker. Skipping these tests as the existing sphere interesction ones
        // are enough to validate that things are working as expected.
        //self.saved_ray = Option::Some(ray.clone());

        vec![]
    }

    pub(crate) fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x(), point.y(), point.z())
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            shape: Shape::new_unit(),
        }
    }

    pub fn new_with_transformation_and_material(
        transformation: Matrix,
        material: Material,
    ) -> Plane {
        Plane {
            shape: Shape::new(transformation, material),
        }
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let result;
        if ray.direction.y().abs() < crate::domain::EPSILON {
            result = Vec::new()
        } else {
            let t = -ray.origin.y() / ray.direction.y();
            result = vec![t];
        }
        result
    }

    pub(crate) fn local_normal_at(&self, _point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
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
    fn new_unit() -> Sphere {
        Sphere {
            origin: Sphere::ORIGIN,
            //radius: UNIT,
            shape: Shape::new_unit(),
        }
    }

    // constructor w/ initial transformation matrix
    fn new(transformation: Matrix) -> Sphere {
        Sphere {
            shape: Shape {
                transformation,
                ..Shape::new_unit()
            },
            ..Sphere::new_unit()
        }
    }

    // constructor w/ material argument
    fn new_material(material: Material) -> Sphere {
        Sphere {
            shape: Shape {
                material,
                ..Shape::new_unit()
            },
            ..Sphere::new_unit()
        }
    }

    // Finds intersections of ray against sphere instance
    fn local_intersect(&self, localized_ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = &localized_ray.origin - &self.origin;
        let a: f64 = localized_ray
            .direction
            .dot_product(&localized_ray.direction);
        let b: f64 = 2.0 * localized_ray.direction.dot_product(&sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
        let discriminant: f64 = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            Vec::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec![t1, t2]
        }
    }

    // Computes the normal at given point.
    fn local_normal_at(&self, point: &Point) -> Vector {
        point - &Sphere::ORIGIN
    }
}
