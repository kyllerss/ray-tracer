use crate::domain::camera::Camera;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersections};
use crate::domain::light::Light;
use crate::domain::object::Object;
use crate::domain::ray::Ray;
use crate::domain::Point;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct World {
    pub objects: Vec<Object>,
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
    pub fn add_object(&mut self, obj: Object) -> &Self {
        self.objects.push(obj);
        self
    }

    // Returns all intersections for given ray in world's objects.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut ints = Intersections::new();
        self.objects
            .iter()
            .for_each(|s| ints.append(s.intersect(&ray)));
        ints
    }

    // Calculates shade hit for the given computations
    pub fn shade_hit(&self, comp: &Computations, iteration: usize) -> Color {
        let in_shadow = self.is_shadowed(&comp.over_point);

        let surface = Light::lighting(
            &comp.object.shape().material,
            &comp.object,
            self.light_source.as_ref().unwrap(),
            &comp.over_point,
            &comp.eye_v,
            &comp.normal_v,
            in_shadow,
        );

        let reflected = self.reflected_color(comp, iteration);
        &surface + &reflected
    }

    // calculates color at a given point
    pub fn color_at(&self, r: &Ray, iteration: usize) -> Color {
        // find intersections
        let mut ints = self.intersect(r);

        let result: Color;

        if let Some(intersection) = ints.hit() {
            let comps = Computations::prepare_computations(&intersection, r);
            result = self.shade_hit(&comps, iteration);
        } else {
            result = Color::BLACK;
        }

        result
    }

    // renders world based on provided camera
    // _logger fix for multi-threading comes from: https://users.rust-lang.org/t/how-to-send-function-closure-to-another-thread/43549
    pub fn render(
        &self,
        camera: &Camera,
        _logger: Arc<dyn Fn(usize, usize) -> () + Send + Sync>,
    ) -> Canvas {
        let total_size = camera.vsize * camera.hsize;
        //let mut results: Vec<(usize, usize, Color)> = Vec::with_capacity(total_size);

        // track iterations for logging
        let itr_counter = AtomicUsize::new(0);

        let iteration_max = 10;

        // compute pixels
        let mut results = (0..camera.vsize)
            .into_par_iter()
            .enumerate()
            .flat_map(move |(_i, y)| {
                let mut r: Vec<(usize, usize, Color)> = Vec::with_capacity(camera.hsize);
                for x in 0..camera.hsize {
                    let ray = camera.ray_for_pixel(x, y);
                    let color = self.color_at(&ray, iteration_max);
                    r.push((x, y, color));
                }

                // log increment
                let size = itr_counter.fetch_add(camera.hsize, Ordering::Relaxed);
                let logger = Arc::clone(&_logger);
                logger(size + camera.hsize, total_size);

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

    // performs reflection calculations
    pub fn reflected_color(&self, comps: &Computations, iteration: usize) -> Color {
        if iteration == 0 || comps.object.shape().material.reflective == 0.0 {
            Color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point.clone(), comps.reflect_v.clone());
            let color = self.color_at(&reflect_ray, iteration - 1);
            &color * comps.object.shape().material.reflective as f32
        }
    }
}
