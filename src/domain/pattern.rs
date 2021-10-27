use crate::domain::color::Color;
use crate::domain::matrix::Matrix;
use crate::domain::object::Object;
use crate::domain::Point;

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    STRIPED {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
    GRADIENT {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
}

// pub struct TwoColorBuilder {
//     a: Color,
//     b: Color,
//     transformation: Option<Matrix>,
// }
//
// impl TwoColorBuilder {
//     pub fn transformation(&mut self, t: Matrix) -> &mut TwoColorBuilder {
//         self.transformation = Option::Some(t);
//         self
//     }
// }
//
// pub struct StripedBuilder(TwoColorBuilder);
//
// impl StripedBuilder {
//     pub fn build(&self) -> Pattern {
//         Pattern::STRIPED {
//             a: self.a,
//             b: self.b,
//             transformation: self
//                 .transformation
//                 .unwrap_or(crate::domain::matrix::IDENTITY.clone()),
//         }
//     }
// }
//
// impl Deref for StripedBuilder {
//     type Target = TwoColorBuilder;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl DerefMut for StripedBuilder {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
//
// impl From<&mut TwoColorBuilder> for StripedBuilder {
//     fn from(b: &mut TwoColorBuilder) -> Self {
//         StripedBuilder(b)
//     }
// }

impl Pattern {
    // constructor
    pub fn new_striped(a: Color, b: Color, transformation: Matrix) -> Pattern {
        Pattern::STRIPED {
            a,
            b,
            transformation,
        }
    }

    pub fn new_gradient(a: Color, b: Color, transformation: Matrix) -> Pattern {
        Pattern::GRADIENT {
            a,
            b,
            transformation,
        }
    }

    // TODO Turn this into a trait method to obviate pattern matching
    pub fn transformation(&self) -> &Matrix {
        match self {
            Pattern::STRIPED { transformation, .. } => transformation,
            Pattern::GRADIENT { transformation, .. } => transformation,
        }
    }

    // calculates color for the given pattern at the given point
    pub fn color_at(&self, object: &Object, world_point: &Point) -> Color {
        // convert pattern to object and pattern orientations
        let object_point = &object.shape().transformation.inverse().unwrap() * world_point;
        let pattern_point = &self.transformation().inverse().unwrap() * &object_point;

        match &self {
            Pattern::STRIPED { a, b, .. } => color_at_striped(a, b, &pattern_point),
            Pattern::GRADIENT { a, b, .. } => color_at_gradient(a, b, &pattern_point),
        }
    }
}

// striped colors alternating across x axis
fn color_at_striped<'a, 'b>(a: &'a Color, b: &'a Color, point: &'b Point) -> Color {
    if point.x().floor() % 2.0 == 0.0 {
        a.clone()
    } else {
        b.clone()
    }
}

// gradient colors across x axis
fn color_at_gradient<'a, 'b>(a: &'a Color, b: &'a Color, point: &'b Point) -> Color {
    let distance = b - a;
    let fraction = point.x() as f32 - point.x().floor() as f32;
    a + &(&distance * fraction)
}
