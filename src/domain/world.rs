use crate::domain::camera::Camera;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersections};
use crate::domain::light::Light;
use crate::domain::object::Sphere;
use crate::domain::ray::Ray;
use crate::domain::Point;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut ints = Intersections::new();
        self.objects
            .iter()
            .for_each(|s| ints.push_all(s.intersect(&ray)));
        ints
    }

    // Calculates shade hit for the given computations
    pub fn shade_hit(&self, comp: &Computations) -> Color {
        let in_shadow = self.is_shadowed(&comp.over_point);

        Light::lighting(
            &comp.object.shape.material,
            self.light_source.as_ref().unwrap(),
            &comp.over_point,
            &comp.eye_v,
            &comp.normal_v,
            in_shadow,
        )
    }

    // calculates color at a given point
    pub fn color_at(&self, r: &Ray) -> Color {
        // find intersections
        let mut ints = self.intersect(r);

        let result: Color;

        if let Some(intersection) = ints.hit() {
            let comps = Computations::prepare_computations(&intersection, r);
            result = self.shade_hit(&comps);
        } else {
            result = Color::BLACK;
        }

        result
    }

    // renders world based on provided camera
    pub fn render(&self, camera: &Camera, _logger: &dyn Fn(usize, usize) -> ()) -> Canvas {
        let total_size = camera.vsize * camera.hsize;
        //let mut results: Vec<(usize, usize, Color)> = Vec::with_capacity(total_size);

        // track iterations for logging
        let itr_counter = AtomicUsize::new(0);

        // compute pixels
        let mut results = (0..camera.vsize)
            .into_par_iter()
            .enumerate()
            .flat_map(|(_i, y)| {
                let mut r: Vec<(usize, usize, Color)> = Vec::with_capacity(camera.hsize);
                for x in 0..camera.hsize {
                    let ray = camera.ray_for_pixel(x, y);
                    let color = self.color_at(&ray);
                    r.push((x, y, color));
                }

                // log increment
                let size = itr_counter.fetch_add(camera.hsize, Ordering::Relaxed);
                //logger(size + camera.hsize, total_size);
                println!(
                    "{}/{} ({}%)",
                    size + camera.hsize,
                    total_size,
                    ((size as f32 + camera.hsize as f32) / total_size as f32) * 100_f32
                );

                // return value
                r
            })
            .collect::<Vec<(usize, usize, Color)>>();

        // apply computed values to canvas
        let mut canvas = Canvas::new(camera.hsize, camera.vsize, Color::BLACK);
        results.drain(..).for_each(|(x, y, color)| {
            canvas.render(x, y, color);
        });
        canvas
    }

    // determines if point is shadowed
    pub fn is_shadowed(&self, p: &Point) -> bool {
        let v = &self.light_source.unwrap().position - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(p.clone(), direction);
        let mut intersections = self.intersect(&r);
        let h = intersections.hit();
        if h.is_some() && h.unwrap().distance < distance {
            true
        } else {
            false
        }
    }
}
