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
    RINGED {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
    CHECKERED {
        a: Color,
        b: Color,
        transformation: Matrix,
    },
    NULL {
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

    pub fn new_ringed(a: Color, b: Color, transformation: Matrix) -> Pattern {
        Pattern::RINGED {
            a,
            b,
            transformation,
        }
    }

    pub fn new_checkered(a: Color, b: Color, transformation: Matrix) -> Pattern {
        Pattern::CHECKERED {
            a,
            b,
            transformation,
        }
    }

    pub fn new_null() -> Pattern {
        Pattern::NULL {
            transformation: crate::domain::matrix::IDENTITY.clone(),
        }
    }

    // TODO Turn this into a trait method to obviate pattern matching
    pub fn transformation(&self) -> &Matrix {
        match self {
            Pattern::STRIPED { transformation, .. } => transformation,
            Pattern::GRADIENT { transformation, .. } => transformation,
            Pattern::RINGED { transformation, .. } => transformation,
            Pattern::CHECKERED { transformation, .. } => transformation,
            Pattern::NULL { transformation, .. } => transformation,
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
            Pattern::RINGED { a, b, .. } => color_at_ringed(a, b, &pattern_point),
            Pattern::CHECKERED { a, b, .. } => color_at_checkered(a, b, &pattern_point),
            Pattern::NULL { .. } => color_at_null(&pattern_point),
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

// ringed colors across x axis
fn color_at_ringed<'a, 'b>(a: &'a Color, b: &'a Color, point: &'b Point) -> Color {
    let first = (point.x().powi(2) + point.z().powi(2)).sqrt().floor() % 2.0 == 0.0;
    if first {
        a.clone()
    } else {
        b.clone()
    }
}

// ringed colors across x axis
fn color_at_checkered<'a, 'b>(a: &'a Color, b: &'a Color, point: &'b Point) -> Color {
    let first = (point.x().floor() + point.y().floor() + point.z().floor()) % 2.0 == 0.0;
    if first {
        a.clone()
    } else {
        b.clone()
    }
}

fn color_at_null(point: &Point) -> Color {
    Color::new(point.x() as f32, point.y() as f32, point.z() as f32)
}
