use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersections};
use crate::domain::light::Light;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: Option<Light>,
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

    // Calculates shade hit for the given computations
    pub fn shade_hit(&self, comp: &Computations) -> Color {
        Light::lighting(
            &comp.object.material,
            self.light_source.as_ref().unwrap(),
            &comp.point,
            &comp.eye_v,
            &comp.normal_v,
        )
    }
}
