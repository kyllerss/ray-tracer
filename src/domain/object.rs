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

#[derive(PartialEq, Debug, Clone)]
pub struct Cube {
    pub shape: Shape,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cylinder {
    pub shape: Shape,
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(PartialEq, Clone)]
pub enum Object {
    Sphere(Sphere),
    Null(Null), // throw-away test implementation
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
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

impl From<Cube> for Object {
    fn from(v: Cube) -> Self {
        Object::Cube(v)
    }
}

impl Object {
    // TODO Define trait that returns these, so that the match is not necessary.
    fn local_intersect(&self, ray: &Ray) -> Intersections {
        let ints = match self {
            Object::Sphere(sphere) => sphere.local_intersect(ray),
            Object::Null(null) => null.local_intersect(ray),
            Object::Plane(plane) => plane.local_intersect(ray),
            Object::Cube(cube) => cube.local_intersect(ray),
            Object::Cylinder(cylinder) => cylinder.local_intersect(ray),
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
            Object::Cube(cube) => cube.local_normal_at(point),
            Object::Cylinder(cylinder) => cylinder.local_normal_at(point),
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape(&self) -> &Shape {
        match self {
            Object::Sphere(sphere) => &sphere.shape,
            Object::Null(null) => &null.shape,
            Object::Plane(plane) => &plane.shape,
            Object::Cube(cube) => &cube.shape,
            Object::Cylinder(cylinder) => &cylinder.shape,
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape_mut(&mut self) -> &mut Shape {
        match self {
            Object::Sphere(sphere) => &mut sphere.shape,
            Object::Null(null) => &mut null.shape,
            Object::Plane(plane) => &mut plane.shape,
            Object::Cube(cube) => &mut cube.shape,
            Object::Cylinder(cylinder) => &mut cylinder.shape,
        }
    }

    // Finds intersections of ray against sphere instance
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let inv_transform = self.shape().transformation.inverse();
        if inv_transform.is_none() {
            panic!("Unexpected non-invertible matrix.");
        }
        let localized_ray = ray.transform(&inv_transform.unwrap());

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

pub struct CubeBuilder {
    shape_builder: ShapeBuilder,
}

impl CubeBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut CubeBuilder {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut CubeBuilder {
        self.shape_builder.material(material);
        self
    }

    pub fn build(&self) -> Cube {
        Cube {
            shape: self.shape_builder.build(),
        }
    }
}

impl Cube {
    pub fn new() -> CubeBuilder {
        CubeBuilder {
            shape_builder: Shape::new("Cube"),
        }
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let (x_tmin, x_tmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (y_tmin, y_tmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (z_tmin, z_tmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());

        let tmin = x_tmin.max(y_tmin.max(z_tmin));
        let tmax = x_tmax.min(y_tmax.min(z_tmax));

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }

    fn check_axis(origin_component: f64, direction_component: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin_component;
        let tmax_numerator = 1.0 - origin_component;

        let (tmin, tmax);
        if direction_component.abs() >= crate::domain::EPSILON {
            tmin = tmin_numerator / direction_component;
            tmax = tmax_numerator / direction_component;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub(crate) fn local_normal_at(&self, point: &Point) -> Vector {
        let max_c = point.x().abs().max(point.y().abs().max(point.z().abs()));

        if max_c == point.x().abs() {
            Vector::new(point.x(), 0.0, 0.0)
        } else if max_c == point.y().abs() {
            Vector::new(0.0, point.y(), 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z())
        }
    }
}

pub struct CylinderBuilder {
    shape_builder: ShapeBuilder,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

impl CylinderBuilder {
    pub fn transformation(&mut self, transformation: Matrix) -> &mut CylinderBuilder {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut CylinderBuilder {
        self.shape_builder.material(material);
        self
    }

    pub fn build(&self) -> Cylinder {
        Cylinder {
            shape: self.shape_builder.build(),
            minimum: self.minimum.unwrap_or(-f64::INFINITY),
            maximum: self.maximum.unwrap_or(f64::INFINITY),
        }
    }
    pub fn minimum(&mut self, minimum: f64) -> &mut CylinderBuilder {
        self.minimum = Option::Some(minimum);
        self
    }

    pub fn maximum(&mut self, maximum: f64) -> &mut CylinderBuilder {
        self.maximum = Option::Some(maximum);
        self
    }
}

impl Cylinder {
    pub fn new() -> CylinderBuilder {
        CylinderBuilder {
            shape_builder: Shape::new("Cylinder"),
            minimum: Option::None,
            maximum: Option::None,
        }
    }

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);
        if a < crate::domain::EPSILON {
            return vec![];
        }

        let b = 2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();
        let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        vec![t0, t1]
    }

    pub(crate) fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x(), 0.0, point.z())
    }
}
