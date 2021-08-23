use crate::domain::object::Sphere;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub distance: f64,
}

impl<'a> Intersection<'a> {
    // constructor
    pub fn new(distance: f64, object: &'a Sphere) -> Intersection {
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
    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.intersections.push(intersection);
    }

    // returns number of intersections
    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    // // returns hit from all intersections
    // pub fn hit(&self) -> Option<&Intersection<'a>> {
    //     self.intersections.peek()
    // }

    // pops minimal item from heap
    pub fn hit(&mut self) -> Option<Intersection> {
        while let Some(intersection) = self.intersections.pop() {
            let valid = !intersection.distance.is_infinite() && !intersection.distance.is_nan();
            if !valid || intersection.distance < 0.0 {
                continue;
            };
            return Some(intersection);
        }

        None
    }
}
