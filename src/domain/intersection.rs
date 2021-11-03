use crate::domain::object::Object;
use crate::domain::ray::Ray;
use crate::domain::{Id, Point, Vector};
use linked_hash_map::LinkedHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub distance: f64,
}

impl<'a> Intersection<'a> {
    // constructor
    pub fn new(distance: f64, object: &'a Object) -> Intersection {
        Intersection { object, distance }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
        // self.distance.partial_cmp(&other.distance)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
        // let d1_valid = !self.distance.is_nan() && !self.distance.is_infinite();
        // let d2_valid = !other.distance.is_nan() && !other.distance.is_infinite();
        // if d1_valid && d2_valid {
        //     self.distance == other.distance
        // } else {
        //     false
        // }
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let partial_cmp = self.partial_cmp(other);
        match partial_cmp {
            None => {
                let d1_valid = !self.distance.is_nan() && !self.distance.is_infinite();
                let d2_valid = !other.distance.is_nan() && !other.distance.is_infinite();
                if d1_valid == d2_valid {
                    return Ordering::Equal;
                }
                if d1_valid && !d2_valid {
                    return Ordering::Less;
                }
                Ordering::Greater
            }
            Some(cmp) => cmp,
        }
    }
}

#[derive(Clone)]
pub struct Intersections<'a> {
    intersections: BinaryHeap<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    // constructor
    pub fn new() -> Intersections<'a> {
        Intersections {
            intersections: BinaryHeap::new(),
        }
    }

    // Takes ownership of intersection
    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.intersections.push(intersection);
    }

    // adds all intersections into data structure
    pub fn append(&mut self, ints: Intersections<'a>) {
        if ints.is_empty() {
            return ();
        }
        let mut b = ints.intersections;
        self.intersections.append(&mut b);
    }

    // returns number of intersections
    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    // determines if there is a hit
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn inner_hit(&mut self, validate: bool) -> Option<Intersection> {
        while let Some(intersection) = self.intersections.pop() {
            if validate {
                let valid = !intersection.distance.is_infinite() && !intersection.distance.is_nan();
                if !valid || intersection.distance < 0.0 {
                    continue;
                };
            }
            return Some(intersection);
        }

        None
    }

    // returns first item (regardless of sign (negative/positive)
    pub fn hit_unchecked(&mut self) -> Option<Intersection> {
        self.inner_hit(false)
    }

    // pops minimal item from heap
    pub fn hit(&mut self) -> Option<Intersection> {
        self.inner_hit(true)
    }
}

pub struct Computations<'a> {
    pub distance: f64,
    pub object: &'a Object,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub reflect_v: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computations<'a> {
    // builder
    pub fn new(
        distance: f64,
        object: &'a Object,
        point: Point,
        eye_v: Vector,
        normal_v: Vector,
        inside: bool,
        over_point: Point,
        reflect_v: Vector,
        n1: f64,
        n2: f64,
    ) -> Computations {
        Computations {
            distance,
            object,
            point,
            eye_v,
            normal_v,
            inside,
            over_point,
            reflect_v,
            n1,
            n2,
        }
    }

    // Utility method for pre-computing reusable, frequently-used computations
    pub fn prepare_computations(
        hit_intersection: &'a Intersection,
        ray: &'a Ray,
        all_intersections: Option<&'a Intersections>,
    ) -> Computations<'a> {
        let point = ray.position(hit_intersection.distance);
        let eye_v = -ray.direction;
        let mut normal_v = hit_intersection.object.normal_at(&point);

        let inside;
        if normal_v.dot_product(&eye_v) < 0.0 {
            inside = true;
            normal_v = -normal_v;
        } else {
            inside = false;
        }

        let over_point = &point + &(&normal_v * crate::domain::EPSILON);

        let reflect_v = ray.direction.reflect(&normal_v);

        // refractive index section
        let (n1, n2) =
            Computations::precompute_refractive_indexes(hit_intersection, all_intersections);

        Computations::new(
            hit_intersection.distance,
            hit_intersection.object,
            point,
            eye_v,
            normal_v,
            inside,
            over_point,
            reflect_v,
            n1,
            n2,
        )
    }

    fn precompute_refractive_indexes(
        hit_intersection: &'a Intersection,
        all_intersections: Option<&'a Intersections>,
    ) -> (f64, f64) {
        let (mut n1, mut n2) = (1.0, 1.0);
        let mut containers: LinkedHashMap<&Id, &Object> = LinkedHashMap::new();
        //containers.insert() -> Option<V> (old overlapping value)
        //containers.back() -> Option<&K, &V>
        //containers.remove(&k) -> Option<V>
        //containers.get(&k) -> Option<&V>
        if all_intersections.is_some() {
            for i in &all_intersections.unwrap().intersections {
                if i == hit_intersection {
                    match containers.is_empty() {
                        true => n1 = 1.0,
                        false => {
                            n1 = containers
                                .back()
                                .unwrap()
                                .1
                                .shape()
                                .material
                                .refractive_index()
                        }
                    }
                }

                let obj_key = &i.object.shape().id;
                if containers.contains_key(obj_key) {
                    containers.remove(obj_key);
                } else {
                    containers.insert(obj_key, &i.object);
                }

                if i == hit_intersection {
                    match containers.is_empty() {
                        true => n2 = 1.0,
                        false => {
                            n2 = containers
                                .back()
                                .unwrap()
                                .1
                                .shape()
                                .material
                                .refractive_index()
                        }
                    }

                    break;
                }
            }
        }
        (n1, n2)
    }
}
