use crate::domain::ray::Ray;
use crate::domain::Point;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    origin: Point,
    //radius: f64,
}

// Unit measure for shapes.
//const UNIT: f64 = 1.0;

impl Sphere {
    // constructor
    pub fn new_unit() -> Sphere {
        Sphere {
            origin: Point::new(0.0, 0.0, 0.0),
            //radius: UNIT,
        }
    }

    // Finds intersections of ray against sphere instance
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - self.origin;
        let a: f64 = ray.direction.dot_product(ray.direction);
        let b: f64 = 2.0 * ray.direction.dot_product(sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;
        let discriminant: f64 = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}
