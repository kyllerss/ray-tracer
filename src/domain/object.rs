use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Id, Point, RayTuple, Vector, EPSILON};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Shape<'a> {
    pub id: Id,
    pub transformation: Matrix,
    pub material: Material,
    pub shape_type_name: String,
    parent: Option<*mut Group<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere<'a> {
    pub shape: Shape<'a>,
    pub origin: Point,
    //radius: f64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Null<'a> {
    pub shape: Shape<'a>,
    // pub saved_ray: Option<Ray>, // for unit testing
}

#[derive(PartialEq, Debug, Clone)]
pub struct Plane<'a> {
    pub shape: Shape<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cube<'a> {
    pub shape: Shape<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cylinder<'a> {
    pub shape: Shape<'a>,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cone<'a> {
    pub shape: Shape<'a>,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Group<'a> {
    pub shape: Shape<'a>,
    pub children: Vec<Object<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct BaseTriangle<'a> {
    pub shape: Shape<'a>,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub e1: Vector,
    pub e2: Vector,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Triangle<'a> {
    pub base_triangle: BaseTriangle<'a>,
    pub normal: Vector,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SmoothTriangle<'a> {
    pub base_triangle: BaseTriangle<'a>,
    pub n1: Vector,
    pub n2: Vector,
    pub n3: Vector,
}

#[derive(PartialEq, Clone)]
pub enum Object<'a> {
    Sphere(Sphere<'a>),
    Null(Null<'a>), // throw-away test implementation
    Plane(Plane<'a>),
    Cube(Cube<'a>),
    Cylinder(Cylinder<'a>),
    Cone(Cone<'a>),
    Group(Box<Group<'a>>),
    Triangle(Triangle<'a>),
    SmoothTriangle(SmoothTriangle<'a>),
}

impl<'a> Debug for Object<'a> {
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

impl<'a> From<Sphere<'a>> for Object<'a> {
    fn from(v: Sphere<'a>) -> Self {
        Object::Sphere(v)
    }
}

impl<'a> From<Plane<'a>> for Object<'a> {
    fn from(v: Plane<'a>) -> Self {
        Object::Plane(v)
    }
}

impl<'a> From<Null<'a>> for Object<'a> {
    fn from(v: Null<'a>) -> Self {
        Object::Null(v)
    }
}

impl<'a> From<Cube<'a>> for Object<'a> {
    fn from(v: Cube<'a>) -> Self {
        Object::Cube(v)
    }
}

impl<'a> From<Cylinder<'a>> for Object<'a> {
    fn from(v: Cylinder<'a>) -> Self {
        Object::Cylinder(v)
    }
}

impl<'a> From<Cone<'a>> for Object<'a> {
    fn from(v: Cone<'a>) -> Self {
        Object::Cone(v)
    }
}

impl<'a> From<Box<Group<'a>>> for Object<'a> {
    fn from(v: Box<Group<'a>>) -> Self {
        Object::Group(v)
    }
}

impl<'a> From<Triangle<'a>> for Object<'a> {
    fn from(v: Triangle<'a>) -> Self {
        Object::Triangle(v)
    }
}

impl<'a> From<SmoothTriangle<'a>> for Object<'a> {
    fn from(v: SmoothTriangle<'a>) -> Self {
        Object::SmoothTriangle(v)
    }
}

unsafe impl<'a> Sync for Shape<'a> {}
unsafe impl<'a> Send for Shape<'a> {}

impl<'s> Object<'s> {
    // TODO Define trait that returns these, so that the match is not necessary.
    pub(crate) fn local_intersect(&self, ray: &Ray) -> Intersections<'_, 's> {
        let mut ints = match self {
            Object::Sphere(sphere) => sphere.local_intersect(ray, &self),
            Object::Null(null) => null.local_intersect(ray, &self),
            Object::Plane(plane) => plane.local_intersect(ray, &self),
            Object::Cube(cube) => cube.local_intersect(ray, &self),
            Object::Cylinder(cylinder) => cylinder.local_intersect(ray, &self),
            Object::Cone(cone) => cone.local_intersect(ray, &self),
            Object::Group(group) => group.local_intersect(ray, &self),
            Object::Triangle(triangle) => triangle.local_intersect(ray, &self),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.local_intersect(ray, &self),
        };
        let mut result = Intersections::new();
        ints.drain(..).for_each(|int| {
            result.push(int);
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
            Object::Cone(cone) => cone.local_normal_at(point),
            Object::Group(group) => group.local_normal_at(point),
            Object::Triangle(triangle) => triangle.local_normal_at(point),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.local_normal_at(point),
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape(&self) -> &'_ Shape<'s> {
        match self {
            Object::Sphere(sphere) => &sphere.shape,
            Object::Null(null) => &null.shape,
            Object::Plane(plane) => &plane.shape,
            Object::Cube(cube) => &cube.shape,
            Object::Cylinder(cylinder) => &cylinder.shape,
            Object::Cone(cone) => &cone.shape,
            Object::Group(group) => &group.shape,
            Object::Triangle(triangle) => &triangle.base_triangle.shape,
            Object::SmoothTriangle(smooth_triangle) => &smooth_triangle.base_triangle.shape,
        }
    }

    // TODO Define trait that returns these, so that the match is not necessary.
    pub fn shape_mut(&mut self) -> &'_ mut Shape<'s> {
        match self {
            Object::Sphere(sphere) => &mut sphere.shape,
            Object::Null(null) => &mut null.shape,
            Object::Plane(plane) => &mut plane.shape,
            Object::Cube(cube) => &mut cube.shape,
            Object::Cylinder(cylinder) => &mut cylinder.shape,
            Object::Cone(cone) => &mut cone.shape,
            Object::Group(group) => &mut group.shape,
            Object::Triangle(triangle) => &mut triangle.base_triangle.shape,
            Object::SmoothTriangle(smooth_triangle) => &mut smooth_triangle.base_triangle.shape,
        }
    }

    // Finds intersections of ray against sphere instance
    pub fn intersect(&self, ray: &Ray) -> Intersections<'_, 's> {
        let inv_transform = self.shape().transformation.inverse();
        if inv_transform.is_none() {
            panic!("Unexpected non-invertible matrix.");
        }
        let localized_ray = ray.transform(&inv_transform.unwrap());

        self.local_intersect(&localized_ray)
    }

    // Computes the normal at given point.
    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.local_normal_at(&local_point);
        self.normal_to_world(&local_normal)
    }

    // Computes world point relative to object space
    pub fn world_to_object(&self, world_point: &Point) -> Point {
        let point = match self.shape().parent() {
            Option::Some(parent_group) => parent_group.world_to_object(world_point),

            Option::None => world_point.clone(),
        };
        &self.shape().transformation.inverse().unwrap() * &point
    }

    // computes normal taking into consideration potential for object to be embedded in one or more groups
    pub fn normal_to_world(&self, normal: &Vector) -> Vector {
        let n: Vector = self.shape().transformation.inverse().unwrap().transpose() * normal;
        let n = n.normalize();

        match self.shape().parent() {
            Option::Some(parent_group) => parent_group.normal_to_world(&n),
            _ => n,
        }
    }
}

impl<'a> Default for Shape<'a> {
    fn default() -> Self {
        Shape {
            id: Id::new(),
            transformation: crate::domain::matrix::IDENTITY.clone(),
            material: Material::default(),
            shape_type_name: String::default(),
            parent: Option::None,
        }
    }
}

impl<'a> Shape<'a> {
    // default constructor
    pub fn new_unit() -> Self {
        Shape::default()
    }

    pub fn builder(shape_type_name: &str) -> ShapeBuilder {
        ShapeBuilder::new(shape_type_name)
    }

    // Translates raw pointer into Object representation.
    pub fn parent(&self) -> Option<Object<'a>> {
        match self.parent {
            Option::Some(parent_group) => unsafe {
                Option::Some(
                    Box::new((parent_group as *const Group).as_ref().unwrap().clone()).into(),
                )
            },
            _ => Option::None,
        }
    }
}

pub struct ShapeBuilder {
    transformation: Option<Matrix>,
    material: Option<Material>,
    shape_type_name: String,
}

impl<'a> ShapeBuilder {
    pub fn new(shape_type_name: &str) -> Self {
        Self {
            transformation: Option::None,
            material: Option::None,
            shape_type_name: shape_type_name.parse().unwrap(),
        }
    }

    pub fn transformation(&mut self, transformation: Matrix) -> &mut Self {
        self.transformation = Option::Some(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut Self {
        self.material = Option::Some(material);
        self
    }

    pub fn build(&self) -> Shape<'a> {
        Shape {
            id: Id::new(),
            transformation: self
                .transformation
                .clone()
                .unwrap_or(crate::domain::matrix::IDENTITY.clone()),
            material: self.material.clone().unwrap_or(Material::default()),
            shape_type_name: self.shape_type_name.clone(),
            parent: Option::None,
        }
    }
}

pub struct NullBuilder {
    shape_builder: ShapeBuilder,
}

impl<'a> NullBuilder {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Null"),
        }
    }
    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn build(self) -> Null<'a> {
        Null {
            shape: self.shape_builder.build(),
        }
    }
}
impl<'s> Null<'s> {
    pub fn builder() -> NullBuilder {
        NullBuilder::new()
    }
    pub(crate) fn local_intersect<'r>(
        &self,
        _ray: &Ray,
        _wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
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

impl<'a> PlaneBuilder {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Plane"),
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn build(self) -> Plane<'a> {
        Plane {
            shape: self.shape_builder.build(),
        }
    }
}

impl<'s> Plane<'s> {
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::new()
    }

    pub(crate) fn local_intersect<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        let result;
        if ray.direction.y().abs() < crate::domain::EPSILON {
            result = Vec::new()
        } else {
            let t = -ray.origin.y() / ray.direction.y();
            result = vec![Intersection::new(t, wrapped_self)];
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

impl<'a> SphereBuilder {
    pub fn new() -> Self {
        Self {
            origin: Option::Some(Point::ORIGIN),
            //radius: UNIT,
            shape_builder: Shape::builder("Sphere"),
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn origin(mut self, origin: Point) -> Self {
        self.origin = Option::Some(origin);
        self
    }

    pub fn build(self) -> Sphere<'a> {
        Sphere {
            shape: self.shape_builder.build(),
            origin: self.origin.unwrap_or(Point::ORIGIN),
        }
    }
}

impl<'s> Sphere<'s> {
    const ORIGIN: Point = Point {
        ray_tuple: RayTuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
    };

    // constructor w/ no transformation matrix (identify matrix default)
    pub fn builder() -> SphereBuilder {
        SphereBuilder::new()
    }

    // Finds intersections of ray against sphere instance
    fn local_intersect<'r>(
        &self,
        localized_ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
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

            vec![
                Intersection::new(t1, wrapped_self),
                Intersection::new(t2, wrapped_self),
            ]
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

impl<'a> CubeBuilder {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Cube"),
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn build(self) -> Cube<'a> {
        Cube {
            shape: self.shape_builder.build(),
        }
    }
}

impl<'s> Cube<'s> {
    pub fn builder() -> CubeBuilder {
        CubeBuilder::new()
    }

    pub(crate) fn local_intersect<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        let (x_tmin, x_tmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (y_tmin, y_tmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (z_tmin, z_tmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());

        let tmin = x_tmin.max(y_tmin.max(z_tmin));
        let tmax = x_tmax.min(y_tmax.min(z_tmax));

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection::new(tmin, wrapped_self),
                Intersection::new(tmax, wrapped_self),
            ]
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
    closed: Option<bool>,
}

impl<'a> CylinderBuilder {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Cylinder"),
            minimum: Option::None,
            maximum: Option::None,
            closed: Option::None,
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn minimum(mut self, minimum: f64) -> Self {
        self.minimum = Option::Some(minimum);
        self
    }

    pub fn maximum(mut self, maximum: f64) -> Self {
        self.maximum = Option::Some(maximum);
        self
    }

    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = Option::Some(closed);
        self
    }

    pub fn build(self) -> Cylinder<'a> {
        Cylinder {
            shape: self.shape_builder.build(),
            minimum: self.minimum.unwrap_or(-f64::INFINITY),
            maximum: self.maximum.unwrap_or(f64::INFINITY),
            closed: self.closed.unwrap_or(false),
        }
    }
}

impl<'s> Cylinder<'s> {
    pub fn builder() -> CylinderBuilder {
        CylinderBuilder::new()
    }

    pub(crate) fn local_intersect<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

        let mut xs = Vec::new();
        if a > crate::domain::EPSILON {
            let b =
                2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();
            let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
            let disc = b.powi(2) - 4.0 * a * c;

            if disc < 0.0 {
                return vec![];
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, wrapped_self));
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, wrapped_self));
            }
        }

        self.intersect_caps(ray, wrapped_self, &mut xs);

        xs
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
        xs: &mut Vec<Intersection<'r, 's>>,
    ) {
        if !self.closed || ray.direction.y().abs() < crate::domain::EPSILON {
            return;
        }

        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            xs.push(Intersection::new(t, wrapped_self));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if Cylinder::check_cap(ray, t) {
            xs.push(Intersection::new(t, wrapped_self));
        }
    }

    pub(crate) fn local_normal_at(&self, point: &Point) -> Vector {
        let dist = point.x().powi(2) + point.z().powi(2);

        if dist < 1.0 && point.y() >= self.maximum - crate::domain::EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() <= self.minimum + crate::domain::EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(point.x(), 0.0, point.z())
        }
    }
}

pub struct ConeBuilder {
    shape_builder: ShapeBuilder,
    minimum: Option<f64>,
    maximum: Option<f64>,
    closed: Option<bool>,
}

impl<'a> ConeBuilder {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Cone"),
            minimum: Option::None,
            maximum: Option::None,
            closed: Option::None,
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.shape_builder.material(material);
        self
    }

    pub fn build(self) -> Cone<'a> {
        Cone {
            shape: self.shape_builder.build(),
            minimum: self.minimum.unwrap_or(-f64::INFINITY),
            maximum: self.maximum.unwrap_or(f64::INFINITY),
            closed: self.closed.unwrap_or(false),
        }
    }
    pub fn minimum(mut self, minimum: f64) -> Self {
        self.minimum = Option::Some(minimum);
        self
    }

    pub fn maximum(mut self, maximum: f64) -> Self {
        self.maximum = Option::Some(maximum);
        self
    }

    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = Option::Some(closed);
        self
    }
}

impl<'s> Cone<'s> {
    pub fn builder() -> ConeBuilder {
        ConeBuilder::new()
    }

    pub(crate) fn local_intersect<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);
        let b = 2.0 * ray.origin.x() * ray.direction.x() - 2.0 * ray.origin.y() * ray.direction.y()
            + 2.0 * ray.origin.z() * ray.direction.z();

        let a_is_zero = a < crate::domain::EPSILON && a > -crate::domain::EPSILON;
        let b_is_zero = b < crate::domain::EPSILON && b > -crate::domain::EPSILON;

        if a_is_zero && b_is_zero {
            return vec![];
        }

        let c = ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);

        let mut xs = Vec::new();
        if a_is_zero && !b_is_zero {
            let t = -c / (2.0 * b);

            xs.push(Intersection::new(t, wrapped_self));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;

            if disc < 0.0 {
                return vec![];
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, wrapped_self));
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, wrapped_self));
            }
        }

        self.intersect_caps(ray, wrapped_self, &mut xs);

        xs
    }

    fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= radius
    }

    fn intersect_caps<'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
        xs: &mut Vec<Intersection<'r, 's>>,
    ) {
        if !self.closed || ray.direction.y().abs() < crate::domain::EPSILON {
            return;
        }

        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if Cone::check_cap(ray, t, self.minimum.abs()) {
            xs.push(Intersection::new(t, wrapped_self));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if Cone::check_cap(ray, t, self.maximum.abs()) {
            xs.push(Intersection::new(t, wrapped_self));
        }
    }

    pub(crate) fn local_normal_at(&self, point: &Point) -> Vector {
        let dist = point.x().powi(2) + point.z().powi(2);

        if dist < 1.0 && point.y() >= self.maximum - crate::domain::EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() <= self.minimum + crate::domain::EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            let mut y = (point.x().powi(2) + point.z().powi(2)).sqrt();
            if point.y() > 0.0 {
                y = -y;
            }

            Vector::new(point.x(), y, point.z())
        }
    }
}

pub struct GroupBuilder<'a> {
    shape_builder: ShapeBuilder,
    children: Vec<Object<'a>>,
}

impl<'a> GroupBuilder<'a> {
    pub fn new() -> Self {
        Self {
            shape_builder: Shape::builder("Group"),
            children: Vec::new(),
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn build(self) -> Box<Group<'a>> {
        unsafe {
            // set parent reference on children
            let group: *mut Group = Box::into_raw(Box::new(Group {
                shape: self.shape_builder.build(),
                children: self.children,
            }));

            for child in (*group).children.iter_mut() {
                child.shape_mut().parent = Option::Some(group);
            }

            Box::from_raw(group)
        }
    }

    pub fn add_child(mut self, child: Object<'a>) -> Self {
        self.children.push(child);
        self
    }
}

impl<'s> Group<'s> {
    pub fn builder() -> GroupBuilder<'s> {
        GroupBuilder::new()
    }

    pub(crate) fn local_intersect(
        &self,
        ray: &Ray,
        _wrapped_self: &'_ Object<'s>,
    ) -> Vec<Intersection<'_, 's>> {
        self.children
            .iter()
            .map(|c| {
                let mut ints = c.intersect(ray);
                let mut result = Vec::new();
                while let Some(int) = ints.hit_unchecked() {
                    result.push(int);
                }
                result
            })
            .flatten()
            .collect::<Vec<Intersection<'_, 's>>>()
    }

    pub(crate) fn local_normal_at(&self, _point: &Point) -> Vector {
        panic!("Group's local_normal_at called!");
    }
}

pub struct BaseTriangleBuilder {
    p1: Point,
    p2: Point,
    p3: Point,
    e1: Vector,
    e2: Vector,
    shape_builder: ShapeBuilder,
}

pub struct TriangleBuilder {
    base_triangle_builder: BaseTriangleBuilder,
    normal: Vector,
}

impl BaseTriangleBuilder {
    fn new(p1: Point, p2: Point, p3: Point, shape_name: &str) -> Self {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;

        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            shape_builder: ShapeBuilder::new(shape_name),
        }
    }

    pub fn transformation(&mut self, transformation: Matrix) -> &Self {
        self.shape_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &Self {
        self.shape_builder.material(material);
        self
    }

    fn build<'s>(&self) -> BaseTriangle<'s> {
        BaseTriangle {
            p1: self.p1,
            p2: self.p2,
            p3: self.p3,
            e1: self.e1,
            e2: self.e2,
            shape: self.shape_builder.build(),
        }
    }
}

impl<'a> BaseTriangle<'a> {
    pub(crate) fn local_intersect<'r, 's: 'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        let dir_cross_e2 = ray.direction.cross_product(&self.e2);
        let det = self.e1.dot_product(&dir_cross_e2);
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = &ray.origin - &self.p1;
        let u = f * p1_to_origin.dot_product(&dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross_product(&self.e1);
        let v = f * ray.direction.dot_product(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.e2.dot_product(&origin_cross_e1);
        vec![Intersection::new_with_uv(t, wrapped_self, u, v)]
    }
}

impl<'a> TriangleBuilder {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let base_triangle_builder = BaseTriangleBuilder::new(p1, p2, p3, "Triangle");
        let e1 = base_triangle_builder.e1;
        let e2 = &base_triangle_builder.e2;
        let normal = e2.cross_product(&e1).normalize();

        Self {
            base_triangle_builder,
            normal,
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.base_triangle_builder.transformation(transformation);
        self
    }

    pub fn material(&mut self, material: Material) -> &Self {
        self.base_triangle_builder.material(material);
        self
    }

    pub fn build(&self) -> Triangle<'a> {
        Triangle {
            base_triangle: self.base_triangle_builder.build(),
            normal: self.normal,
        }
    }
}

impl<'a> Triangle<'a> {
    // Constructor
    pub fn builder(p1: Point, p2: Point, p3: Point) -> TriangleBuilder {
        TriangleBuilder::new(p1, p2, p3)
    }

    pub(crate) fn local_intersect<'r, 's: 'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        self.base_triangle.local_intersect(ray, wrapped_self)
    }

    pub(crate) fn local_normal_at<'r>(&'r self, _point: &Point) -> Vector {
        self.normal
    }

    pub fn p1(&self) -> Point {
        self.base_triangle.p1
    }

    pub fn p2(&self) -> Point {
        self.base_triangle.p2
    }

    pub fn p3(&self) -> Point {
        self.base_triangle.p3
    }

    pub fn e1(&self) -> Vector {
        self.base_triangle.e1
    }

    pub fn e2(&self) -> Vector {
        self.base_triangle.e2
    }
}

pub struct SmoothTriangleBuilder {
    n1: Vector,
    n2: Vector,
    n3: Vector,
    base_triangle_builder: BaseTriangleBuilder,
}

impl<'a> SmoothTriangleBuilder {
    pub fn new(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        let base_triangle_builder = BaseTriangleBuilder::new(p1, p2, p3, "SmoothTriangle");

        Self {
            n1,
            n2,
            n3,
            base_triangle_builder,
        }
    }

    pub fn transformation(mut self, transformation: Matrix) -> Self {
        self.base_triangle_builder.transformation(transformation);
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.base_triangle_builder.material(material);
        self
    }

    pub fn build(self) -> SmoothTriangle<'a> {
        SmoothTriangle {
            n1: self.n1,
            n2: self.n2,
            n3: self.n3,
            base_triangle: self.base_triangle_builder.build(),
        }
    }
}

impl<'a> SmoothTriangle<'a> {
    // Constructor
    pub fn builder(
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> SmoothTriangleBuilder {
        SmoothTriangleBuilder::new(p1, p2, p3, n1, n2, n3)
    }

    pub(crate) fn local_intersect<'r, 's: 'r>(
        &self,
        ray: &Ray,
        wrapped_self: &'r Object<'s>,
    ) -> Vec<Intersection<'r, 's>> {
        self.base_triangle.local_intersect(ray, wrapped_self)
    }

    pub(crate) fn local_normal_at<'r>(&'r self, _point: &Point) -> Vector {
        self.n1
    }

    pub fn p1(&self) -> Point {
        self.base_triangle.p1
    }

    pub fn p2(&self) -> Point {
        self.base_triangle.p2
    }

    pub fn p3(&self) -> Point {
        self.base_triangle.p3
    }

    pub fn e1(&self) -> Vector {
        self.base_triangle.e1
    }

    pub fn e2(&self) -> Vector {
        self.base_triangle.e2
    }
}
