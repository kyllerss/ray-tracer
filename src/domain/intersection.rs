use crate::domain::object::Renderable;
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// To avoid lifetimes stuff, used Box to encapsulate dyn Renderable
// then moved it over to Rc<>. That didn't work w/ rayon parallelization
// so transfered it to Arc<>, but both Arc and Rc were polluting the general
// API as I was adding it everywhere I needed a simple reference (is that best practice?)

#[derive(Debug)]
pub struct Intersection<'a> {
    pub object: &'a dyn Renderable,
    pub distance: f64,
}

impl<'a> Intersection<'a> {
    // constructor
    pub fn new(distance: f64, object: &'a dyn Renderable) -> Intersection {
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
    pub fn push(mut self, intersection: Intersection<'a>) -> Self {
        self.intersections.push(intersection);
        self
    }

    // adds all intersections into data structure
    pub fn append(&mut self, mut ints: Intersections<'a>) {
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
    pub object: &'a dyn Renderable,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl<'a> Computations<'a> {
    // builder
    pub fn new(
        distance: f64,
        object: &'a dyn Renderable,
        point: Point,
        eye_v: Vector,
        normal_v: Vector,
        inside: bool,
        over_point: Point,
    ) -> Computations {
        Computations {
            distance,
            object,
            point,
            eye_v,
            normal_v,
            inside,
            over_point,
        }
    }

    // Utility method for pre-computing reusable, frequently-used computations
    pub fn prepare_computations(i: &'a Intersection, r: &'a Ray) -> Computations<'a> {
        let point = r.position(i.distance);
        let eye_v = -r.direction;
        let mut normal_v = i.object.normal_at(&point);

        let inside;
        if normal_v.dot_product(&eye_v) < 0.0 {
            inside = true;
            normal_v = -normal_v;
        } else {
            inside = false;
        }

        let over_point = &point + &(&normal_v * crate::domain::EPSILON);

        Computations::new(
            i.distance, i.object, point, eye_v, normal_v, inside, over_point,
        )
    }
}
// pub struct Computations<'a, T: Renderable> {
//     pub distance: f64,
//     pub object: &'a T,
//     pub point: Point,
//     pub eye_v: Vector,
//     pub normal_v: Vector,
//     pub inside: bool,
//     pub over_point: Point,
// }
//
// impl<'a, T: Renderable> Computations<'a, T> {
//     // builder
//     pub fn new(
//         distance: f64,
//         object: &'a T,
//         point: Point,
//         eye_v: Vector,
//         normal_v: Vector,
//         inside: bool,
//         over_point: Point,
//     ) -> Computations<T> {
//         Computations {
//             distance,
//             object,
//             point,
//             eye_v,
//             normal_v,
//             inside,
//             over_point,
//         }
//     }
//
//     // Utility method for pre-computing reusable, frequently-used computations
//     pub fn prepare_computations(i: &'a Intersection, r: &'a Ray) -> Computations<'a, T> {
//         let point = r.position(i.distance);
//         let eye_v = -r.direction;
//         let mut normal_v = i.object.normal_at(&point);
//
//         let inside;
//         if normal_v.dot_product(&eye_v) < 0.0 {
//             inside = true;
//             normal_v = -normal_v;
//         } else {
//             inside = false;
//         }
//
//         let over_point = &point + &(&normal_v * crate::domain::EPSILON);
//
//         Computations::new(
//             i.distance, i.object, point, eye_v, normal_v, inside, over_point,
//         )
//     }
// }
