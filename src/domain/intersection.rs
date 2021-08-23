use crate::domain::object::Sphere;
use std::ops::Index;

#[derive(PartialEq, Debug)]
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

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    // constructor
    pub fn new() -> Intersections<'a> {
        Intersections {
            intersections: Vec::new(),
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
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}
