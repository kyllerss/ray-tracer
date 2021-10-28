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
    pub reflective: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::WHITE,
            ambient: Material::DEFAULT_AMBIENT,
            diffuse: Material::DEFAULT_DIFFUSE,
            specular: Material::DEFAULT_SPECULAR,
            shininess: Material::DEFAULT_SHININESS,
            pattern: None,
            reflective: Material::DEFAULT_REFLECTIVE,
        }
    }
}

pub struct MaterialBuilder {
    color: Option<Color>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    pattern: Option<Pattern>,
    reflective: Option<f64>,
}

impl Material {
    pub const DEFAULT_AMBIENT: f64 = 0.1;
    pub const DEFAULT_DIFFUSE: f64 = 0.9;
    pub const DEFAULT_SPECULAR: f64 = 0.9;
    pub const DEFAULT_SHININESS: f64 = 200.0;
    pub const DEFAULT_REFLECTIVE: f64 = 0.0;

    // builder
    pub fn new() -> MaterialBuilder {
        MaterialBuilder {
            color: Option::None,
            ambient: Option::None,
            diffuse: Option::None,
            specular: Option::None,
            shininess: Option::None,
            pattern: Option::None,
            reflective: Option::None,
        }
    }
}

impl MaterialBuilder {
    // finalizes built instance
    pub fn build(&mut self) -> Material {
        Material {
            color: self.color.unwrap_or(Color::WHITE),
            ambient: self.ambient.unwrap_or(Material::DEFAULT_AMBIENT),
            diffuse: self.diffuse.unwrap_or(Material::DEFAULT_DIFFUSE),
            specular: self.specular.unwrap_or(Material::DEFAULT_SPECULAR),
            shininess: self.shininess.unwrap_or(Material::DEFAULT_SHININESS),
            pattern: self.pattern.clone(),
            reflective: self.reflective.unwrap_or(Material::DEFAULT_REFLECTIVE),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut MaterialBuilder {
        self.color = Some(color);
        self
    }

    pub fn ambient(&mut self, ambient: f64) -> &mut MaterialBuilder {
        self.ambient = Some(ambient);
        self
    }

    pub fn diffuse(&mut self, diffuse: f64) -> &mut MaterialBuilder {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn specular(&mut self, specular: f64) -> &mut MaterialBuilder {
        self.specular = Some(specular);
        self
    }

    pub fn shininess(&mut self, shininess: f64) -> &mut MaterialBuilder {
        self.shininess = Some(shininess);
        self
    }

    pub fn pattern(&mut self, pattern: Pattern) -> &mut MaterialBuilder {
        self.pattern = Some(pattern);
        self
    }

    pub fn reflective(&mut self, reflective: f64) -> &mut MaterialBuilder {
        self.reflective = Some(reflective);
        self
    }
}
