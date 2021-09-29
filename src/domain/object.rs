use crate::domain::intersection::Intersection;
use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::{Point, RayTuple, Vector};

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
    pub origin: Point,
    //radius: f64,
    pub transformation: Matrix,
}

// Unit measure for shapes.
//const UNIT: f64 = 1.0;

impl Sphere {
    const ORIGIN: Point = Point {
        ray_tuple: RayTuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
    };

    // constructor w/ no transformation matrix (identify matrix default)
    pub fn new_unit() -> Sphere {
        Sphere {
            origin: Sphere::ORIGIN,
            //radius: UNIT,
            transformation: crate::domain::matrix::IDENTITY.clone(),
        }
    }

    // constructor w/ initial transformation matrix
    pub fn new(transformation: Matrix) -> Sphere {
        Sphere {
            origin: Sphere::ORIGIN,
            transformation: transformation,
        }
    }

    // Finds intersections of ray against sphere instance
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let inv_sphere_transform = self.transformation.inverse();
        if inv_sphere_transform.is_none() {
            panic!("Unexpected non-invertible matrix.");
        }
        let localized_ray = ray.transform(&inv_sphere_transform.unwrap());

        let sphere_to_ray = &localized_ray.origin - &self.origin;
        let a: f64 = localized_ray
            .direction
            .dot_product(&localized_ray.direction);
        let b: f64 = 2.0 * localized_ray.direction.dot_product(&sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;
        let discriminant: f64 = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
        }
    }

    // Computes the normal at given point.
    pub fn normal_at(&self, point: Point) -> Vector {
        //let v = point - Sphere::ORIGIN;
        let mut st_inv = self.transformation.inverse().unwrap();
        let object_point = &st_inv * &point;
        let object_normal = &object_point - &Sphere::ORIGIN;
        let world_normal = &*st_inv.transpose() * &object_normal;

        world_normal.normalize()
    }
}
