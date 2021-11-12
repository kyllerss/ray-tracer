use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Id, Point, RayTuple, Vector};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Shape {
    pub id: Id,
    pub transformation: Matrix,
    pub material: Material,
    pub shape_type_name: String,
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

#[derive(PartialEq, Clone)]
pub enum Object {
    Sphere(Sphere),
    Null(Null), // throw-away test implementation
    Plane(Plane),
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} - {}",
            self.shape().shape_type_name,
            self.shape().id.id,
            self.shape().material.refractive_index()
        )
    }
}

impl From<Sphere> for Object {
    fn from(v: Sphere) -> Self {
        Object::Sphere(v)
    }
}

impl From<Plane> for Object {
    fn from(v: Plane) -> Self {
        Object::Plane(v)
    }
}

impl From<Null> for Object {
    fn from(v: Null) -> Self {
        Object::Null(v)
    }
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
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            id: Id::new(),
            transformation: crate::domain::matrix::IDENTITY.clone(),
            material: Material::default(),
            shape_type_name: String::default(),
        }
    }
}

impl Shape {
    // default constructor
    pub fn new_unit() -> Shape {
        Shape::default()
    }

    pub fn new(shape_type_name: &str) -> ShapeBuilder {
        ShapeBuilder {
            transformation: Option::None,
            material: Option::None,
            shape_type_name: shape_type_name.parse().unwrap(),
        }
    }
}

pub struct ShapeBuilder {
    transformation: Option<Matrix>,
    material: Option<Material>,
    shape_type_name: String,
}

impl ShapeBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut ShapeBuilder {
        self.transformation = Option::Some(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut ShapeBuilder {
        self.material = Option::Some(material);
        self
    }

    pub fn build(&self) -> Shape {
        Shape {
            id: Id::new(),
            transformation: self
                .transformation
                .clone()
                .unwrap_or(crate::domain::matrix::IDENTITY.clone()),
            material: self.material.clone().unwrap_or(Material::default()),
            shape_type_name: self.shape_type_name.clone(),
        }
    }
}

pub struct NullBuilder {
    shape_builder: ShapeBuilder,
}

impl NullBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut NullBuilder {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut NullBuilder {
        self.shape_builder.material(material);
        self
    }

    pub fn build(&self) -> Null {
        Null {
            shape: self.shape_builder.build(),
        }
    }
}
impl Null {
    pub fn new() -> NullBuilder {
        NullBuilder {
            shape_builder: Shape::new("Null"),
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

pub struct PlaneBuilder {
    shape_builder: ShapeBuilder,
}

impl PlaneBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut PlaneBuilder {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut PlaneBuilder {
        self.shape_builder.material(material);
        self
    }

    pub fn build(&self) -> Plane {
        Plane {
            shape: self.shape_builder.build(),
        }
    }
}

impl Plane {
    pub fn new() -> PlaneBuilder {
        PlaneBuilder {
            shape_builder: Shape::new("Plane"),
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

pub struct SphereBuilder {
    shape_builder: ShapeBuilder,
    origin: Option<Point>,
}

impl SphereBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut SphereBuilder {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut SphereBuilder {
        self.shape_builder.material(material);
        self
    }

    pub fn origin(&mut self, origin: Point) -> &mut SphereBuilder {
        self.origin = Option::Some(origin);
        self
    }

    pub fn build(&self) -> Sphere {
        Sphere {
            shape: self.shape_builder.build(),
            origin: self.origin.unwrap_or(Point::ORIGIN),
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
    pub fn new() -> SphereBuilder {
        SphereBuilder {
            origin: Option::Some(Point::ORIGIN),
            //radius: UNIT,
            shape_builder: Shape::new("Sphere"),
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
