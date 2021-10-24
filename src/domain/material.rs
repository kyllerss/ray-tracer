use crate::domain::color::Color;
use crate::domain::pattern::Pattern;

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub const DEFAULT_AMBIENT: f64 = 0.1;
    pub const DEFAULT_DIFFUSE: f64 = 0.9;
    pub const DEFAULT_SPECULAR: f64 = 0.9;
    pub const DEFAULT_SHININESS: f64 = 200.0;

    // constructor
    pub fn new() -> Material {
        Material {
            color: Color::WHITE,
            ambient: Material::DEFAULT_AMBIENT,
            diffuse: Material::DEFAULT_DIFFUSE,
            specular: Material::DEFAULT_SPECULAR,
            shininess: Material::DEFAULT_SHININESS,
            pattern: None,
        }
    }

    // full constructor
    pub fn new_full(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<Pattern>,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
        }
    }
}
