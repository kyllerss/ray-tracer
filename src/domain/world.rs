use crate::domain::camera::Camera;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::intersection::{Computations, Intersections};
use crate::domain::light::Light;
use crate::domain::object::Object;
use crate::domain::ray::Ray;
use crate::domain::{Id, Point};
use log::Level::Debug;
use log::{debug, info, log_enabled, trace};
use num::traits::Pow;
use rayon::prelude::*;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Determines maximum iteration depth when tracking ray bounces.
const MAX_ITERATIONS: usize = 4;

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
    pub fn shade_hit(&self, comp: &Computations, remaining_iterations: usize) -> Color {
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

        let call_id = Id::new().id;
        // println!(
        //     "call_id {:0>3} -> distance: {}, point: {:?}",
        //     call_id, &comp.distance, &comp.point
        // );

        trace!(
            "{:width$} COLOR_AT ({}) -> down the rabbit hole!",
            " ",
            remaining_iterations,
            width = (MAX_ITERATIONS - remaining_iterations) * 3
        );
        // let reflected = Color::BLACK; //self.reflected_color(comp, remaining_iterations);
        let refracted = self.refracted_color(comp, remaining_iterations);
        let reflected = self.reflected_color(comp, remaining_iterations);

        let result = &(&surface + &reflected) + &refracted;

        trace!(
            "{:width$} COLOR_AT ({}) -> total: {:?}, reflected: {:?}, refracted: {:?}",
            " ",
            remaining_iterations,
            (result.red, result.green, result.blue),
            (reflected.red, reflected.green, reflected.blue),
            (refracted.red, refracted.green, refracted.blue),
            width = (MAX_ITERATIONS - remaining_iterations) * 3
        );

        // println!(
        //     "remaining {} -> surface {:?}, refracted {:?}, result: {:?}",
        //     remaining_iterations, &surface, &refracted, &result
        // );

        result
    }

    // calculates color at a given point
    pub fn color_at(&self, r: &Ray, remaining_iterations: usize) -> Color {
        // find intersections
        let mut ints = self.intersect(r);
        let original_ints = ints.clone();

        match ints.hit() {
            Some(intersection) => {
                let comps = Computations::prepare_computations(
                    &intersection,
                    r,
                    Option::Some(&original_ints),
                );
                self.shade_hit(&comps, remaining_iterations)
            }
            None => {
                trace!(
                    "{:width$}   NO INTERSECTIONS ({}) -> EOL - Black",
                    " ",
                    remaining_iterations,
                    width = (MAX_ITERATIONS - remaining_iterations) * 3
                );
                Color::BLACK
            }
        }
    }

    // renders world based on provided camera
    // _logger fix for multi-threading comes from: https://users.rust-lang.org/t/how-to-send-function-closure-to-another-thread/43549
    pub fn render(
        &self,
        camera: &Camera,
        logger: Arc<dyn Fn(usize, usize) -> () + Send + Sync>,
    ) -> Canvas {
        let total_size = camera.vsize * camera.hsize;
        //let mut results: Vec<(usize, usize, Color)> = Vec::with_capacity(total_size);

        // track iterations for logging
        let itr_counter = AtomicUsize::new(0);

        let iteration_max = MAX_ITERATIONS;

        // compute pixels
        let mut results = (0..camera.vsize)
            .into_par_iter()
            .enumerate()
            .flat_map(move |(_i, y)| {
                let mut r: Vec<(usize, usize, Color)> = Vec::with_capacity(camera.hsize);
                for x in 0..camera.hsize {
                    if log_enabled!(Debug) {
                        if x != 148 || y != 159 {
                            // reflection
                            continue;
                        }
                        // if x != 147 || y != 153 {
                        //     // no reflection
                        //     continue;
                        // }
                        // if x != 125 || y != 125 {
                        //     continue;
                        // }
                    }
                    // println!("Rendering pixel ({}, {})...", x, y);
                    let ray = camera.ray_for_pixel(x, y);
                    // println!("---- Calling from world.render(...) ----");
                    // let _ = stdout().flush();
                    let color = self.color_at(&ray, iteration_max);
                    debug!("Final color: {:?}", &color);
                    r.push((x, y, color));
                }

                // log increment
                let size = itr_counter.fetch_add(camera.hsize, Ordering::Relaxed);
                let log = Arc::clone(&logger);
                log(size + camera.hsize, total_size);

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
        match intersections.hit() {
            Option::Some(int) => int.distance < distance,
            Option::None => false,
        }
    }

    // performs reflection calculations
    pub fn reflected_color(&self, comps: &Computations, remaining_iterations: usize) -> Color {
        if remaining_iterations <= 0 || comps.object.shape().material.reflective == 0.0 {
            // println!(
            //     "   iteration: {} -> REFLECTED_color early exit...",
            //     remaining_iterations
            // );
            trace!(
                "{:width$}   REFLECTED ({}) -> EOL - Black",
                " ",
                remaining_iterations,
                width = (MAX_ITERATIONS - remaining_iterations) * 3
            );
            Color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point.clone(), comps.reflect_v.clone());
            // println!("---- Calling from world.reflected_color(...) ----");
            // let _ = stdout().flush();
            trace!(
                "{:width$}   REFLECTED ({}) -> color_at",
                " ",
                remaining_iterations,
                width = (MAX_ITERATIONS - remaining_iterations) * 3
            );
            let reflected_color = self.color_at(&reflect_ray, remaining_iterations - 1);
            trace!(
                "{:width$}   REFLECTED ({}) -> returning {:?}",
                " ",
                remaining_iterations,
                (
                    reflected_color.red,
                    reflected_color.green,
                    reflected_color.blue
                ),
                width = (MAX_ITERATIONS - remaining_iterations) * 3,
            );
            // println!(
            //     "   iteration: {} -> REFLECTED_color normal exit...",
            //     remaining_iterations
            // );
            &reflected_color * comps.object.shape().material.reflective as f32
        }
    }

    // performs refracted color calculation
    pub fn refracted_color(&self, comps: &Computations, remaining_iterations: usize) -> Color {
        if remaining_iterations <= 0 || comps.object.shape().material.transparency == 0.0 {
            trace!(
                "{:width$}   REFRACTED ({}) -> EOL - Black",
                " ",
                remaining_iterations,
                width = (MAX_ITERATIONS - remaining_iterations) * 3
            );
            // println!(
            //     "   iteration: {} -> refracted_color early exit...",
            //     remaining_iterations
            // );
            Color::BLACK
        } else {
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eye_v.dot_product(&comps.normal_v);
            let sin2_t: f64 = n_ratio.pow(2) * (1.0 - cos_i.pow(2));

            if sin2_t > 1.0 {
                // println!(
                //     "   iteration: {} -> refracted_color second early exit...",
                //     remaining_iterations
                // );
                trace!(
                    "{:width$}   REFRACTED ({}) -> sin2_t > 1 - Black",
                    " ",
                    remaining_iterations,
                    width = (MAX_ITERATIONS - remaining_iterations) * 3
                );
                return Color::BLACK;
            }

            let cos_t: f64 = (1_f64 - sin2_t).sqrt();
            let direction =
                &(&comps.normal_v * (n_ratio * cos_i - cos_t)) - &(&comps.eye_v * n_ratio);
            let refract_ray = Ray::new(comps.under_point.clone(), direction);

            // println!("---- Calling from world.refracted_color(...) ----");
            //let _ = stdout().flush();
            trace!(
                "{:width$}   REFRACTED ({}) -> calling color_at",
                " ",
                remaining_iterations,
                width = (MAX_ITERATIONS - remaining_iterations) * 3
            );
            let refracted_color = &self.color_at(&refract_ray, remaining_iterations - 1)
                * comps.object.shape().material.transparency as f32;

            trace!(
                "{:width$}   REFRACTED ({}) -> returning {:?}",
                " ",
                remaining_iterations,
                (
                    refracted_color.red,
                    refracted_color.green,
                    refracted_color.blue
                ),
                width = (MAX_ITERATIONS - remaining_iterations) * 3,
            );
            // println!(
            //     "   Refracted remaining {:?} -> returning color_at with raw {:?}",
            //     remaining_iterations, &refracted_color
            // );
            // ---------
            // println!("== remaining {} ==", remaining_iterations);
            // println!("> n1: {}", &comps.n1);
            // println!("> n2: {}", &comps.n2);
            // println!("> eyev: {:?}", &comps.eye_v);
            // println!("> normalv: {:?}", &comps.normal_v);
            // println!("> under point: {:?}", &comps.under_point);
            // println!("> n_ratio: {}", n_ratio);
            // println!("> cos_i: {}", cos_i);
            // println!("> sin2_t: {}", sin2_t);
            // println!("> cos_t: {}", cos_t);
            // println!("> refracted direction: {:?}", direction);
            // println!("> refracted_color: {:?}", refracted_color);
            // let _ = stdout().flush();
            //
            // ---------
            // println!(
            //     "   iteration: {} -> refracted_color normal exit...",
            //     remaining_iterations
            // );

            refracted_color
        }
    }
}
