use crate::domain::intersection::{Intersection, Intersections};
use crate::domain::light::Light;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: Option<Light>,
}

pub struct Computations<'a> {
    pub distance: f64,
    pub object: &'a Sphere,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
}

impl<'a> Computations<'a> {
    // builder
    pub fn new(
        distance: f64,
        object: &'a Sphere,
        point: Point,
        eye_v: Vector,
        normal_v: Vector,
    ) -> Computations {
        Computations {
            distance,
            object,
            point,
            eye_v,
            normal_v,
        }
    }

    // Utility method for pre-computing reusable, frequently-used computations
    pub fn prepare_computations(i: &'a Intersection, r: &'a Ray) -> Computations<'a> {
        let point = r.position(i.distance);
        let eye_v = -r.direction;
        let normal_v = i.object.normal_at(&point);

        Computations::new(i.distance, i.object, point, eye_v, normal_v)
    }
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light_source: Option::None,
        }
    }

    // adds object to world
    pub fn add_object(&mut self, obj: Sphere) -> &Self {
        self.objects.push(obj);
        self
    }

    // Returns all intersections for given ray in world's objects.
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut r = Intersections::new();
        self.objects
            .iter()
            .for_each(|s| r.push_all(s.intersect(&ray)));
        r
    }
}
