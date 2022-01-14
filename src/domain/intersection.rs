use crate::domain::object::Object;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'r, 's> {
    pub object: &'r Object<'s>,
    pub distance: f64,
}

impl<'r, 's: 'r> Intersection<'r, 's> {
    // constructor
    pub fn new(distance: f64, object: &'r Object<'s>) -> Intersection<'r, 's> {
        Intersection { object, distance }
    }
}

impl<'r, 's> PartialOrd for Intersection<'r, 's> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // smallest is GT - contrary to self.partial_cmp(other)
        other.distance.partial_cmp(&self.distance)
    }
}

impl<'r, 's: 'r> PartialEq for Intersection<'r, 's> {
    fn eq<'a>(&'a self, other: &'a Self) -> bool {
        crate::domain::epsilon_eq(self.distance, other.distance)
            && self.object.shape().id == other.object.shape().id
        //self.distance == other.distance

        // let d1_valid = !self.distance.is_nan() && !self.distance.is_infinite();
        // let d2_valid = !other.distance.is_nan() && !other.distance.is_infinite();
        // if d1_valid && d2_valid {
        //     self.distance == other.distance
        // } else {
        //     false
        // }
    }
}

impl<'r, 's> Eq for Intersection<'r, 's> {}

impl<'r, 's> Ord for Intersection<'r, 's> {
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
pub struct Intersections<'r, 's> {
    intersections: BinaryHeap<Intersection<'r, 's>>,
}

impl<'r, 's> Debug for Intersections<'r, 's> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tmp = self.clone();
        let _ = write!(f, "Intersections [");
        while let Some(int) = tmp.intersections.pop() {
            let _ = write!(f, "{:?}, ", int);
        }
        write!(f, "]")
    }
}

impl<'r, 's> Intersections<'r, 's> {
    // constructor
    pub fn new() -> Intersections<'r, 's> {
        Intersections {
            intersections: BinaryHeap::new(),
        }
    }

    // Takes ownership of intersection
    pub fn push(&mut self, intersection: Intersection<'r, 's>) {
        self.intersections.push(intersection);
    }

    // adds all intersections into data structure
    pub fn append(&mut self, ints: Intersections<'r, 's>) {
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

    fn inner_hit(&mut self, validate: bool) -> Option<Intersection<'r, 's>> {
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
    pub fn hit_unchecked(&mut self) -> Option<Intersection<'r, 's>> {
        self.inner_hit(false)
    }

    // pops minimal item from heap
    pub fn hit(&mut self) -> Option<Intersection<'r, 's>> {
        self.inner_hit(true)
    }
}

pub struct Computations<'r, 's> {
    pub distance: f64,
    pub object: &'r Object<'s>,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub reflect_v: Vector,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Point,
}

impl<'r, 's: 'r> Computations<'r, 's> {
    // builder
    pub fn new(
        distance: f64,
        object: &'r Object<'s>,
        point: Point,
        eye_v: Vector,
        normal_v: Vector,
        inside: bool,
        over_point: Point,
        reflect_v: Vector,
        n1: f64,
        n2: f64,
        under_point: Point,
    ) -> Computations<'r, 's> {
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
            under_point,
        }
    }

    // Utility method for pre-computing reusable, frequently-used computations
    pub fn prepare_computations(
        hit_intersection: &'r Intersection<'r, 's>,
        ray: &Ray,
        all_intersections: Option<&'r Intersections<'r, 's>>,
    ) -> Computations<'r, 's> {
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

        let epsilon_scaled_normal = &normal_v * crate::domain::EPSILON;
        let over_point = &point + &epsilon_scaled_normal;
        let under_point = &point - &epsilon_scaled_normal;

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
            under_point,
        )
    }

    fn precompute_refractive_indexes(
        hit_intersection: &'r Intersection<'r, 's>,
        all_intersections: Option<&'r Intersections<'r, 's>>,
    ) -> (f64, f64) {
        if all_intersections.is_none() {
            return (1.0, 1.0);
        }

        // println!("Hit intersection: {:?}", &hit_intersection);
        // println!("All intersections: {:?}", &all_intersections.unwrap());

        let (mut n1, mut n2) = (1.0, 1.0);
        let mut containers: Vec<&Object> = Vec::new();
        //let mut container_keys: HashSet<&Id> = HashSet::new();

        let mut tmp_heap = all_intersections.unwrap().clone().intersections;
        while let Some(i) = tmp_heap.pop() {
            let is_hit = i == *hit_intersection;

            if is_hit {
                match containers.is_empty() {
                    true => {
                        n1 = 1.0;
                        // println!("Hit on {:?} -> n1: {} - empty", &i.object, n1)
                    }

                    false => {
                        let obj = containers.last().unwrap();
                        // println!(
                        //     "Hit on {:?} -> n1: {:?}",
                        //     &i.object,
                        //     obj.shape().material.refractive_index()
                        // );
                        n1 = obj.shape().material.refractive_index()
                    }
                }
            }
            // let obj_key = &i.object.shape().id;
            if containers.contains(&i.object) {
                // println!("Removing {:?} from {:?}", &i.object, &containers);
                // let removed_item = containers.pop().unwrap();
                containers.retain(|item| item.shape().id != i.object.shape().id);
                // println!("Containers is now {:?}", &containers);
                // println!(
                //     "Removed {} {:?}",
                //     &i.object.shape().shape_type_name,
                //     &i.object.shape().id
                // );
            } else {
                // println!("Appending {:?} to {:?}", &i.object, &containers);
                containers.push(i.object);
                // println!("Containers is now {:?}", &containers);
            }

            if is_hit {
                match containers.is_empty() {
                    true => {
                        n2 = 1.0;
                        // println!("Hit on {:?} -> n2: {} - empty", &i.object, n2)
                    }

                    false => {
                        let obj = containers.last().unwrap();
                        // println!(
                        //     "Hit on {:?} -> n2: {:?}",
                        //     &i.object,
                        //     obj.shape().material.refractive_index()
                        // );
                        n2 = obj.shape().material.refractive_index()
                    }
                }

                break;
            }
        }
        // println!("Exiting: ({}, {})", n1, n2);
        // let _ = stdout().flush();
        (n1, n2)
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_v.dot_product(&self.normal_v);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
