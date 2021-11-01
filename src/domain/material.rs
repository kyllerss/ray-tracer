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
    pub transparency: f64,
    pub substance: Substance,
    pub refractive_index_override: f64,
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
            transparency: Material::DEFAULT_TRANSPARENCY,
            substance: Material::DEFAULT_SUBSTANCE,
            refractive_index_override: Material::DEFAULT_SUBSTANCE.refractive_index(),
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
    transparency: Option<f64>,
    substance: Option<Substance>,
    refractive_index_override: Option<f64>,
}

impl Material {
    pub const DEFAULT_AMBIENT: f64 = 0.1;
    pub const DEFAULT_DIFFUSE: f64 = 0.9;
    pub const DEFAULT_SPECULAR: f64 = 0.9;
    pub const DEFAULT_SHININESS: f64 = 200.0;
    pub const DEFAULT_REFLECTIVE: f64 = 0.0;
    pub const DEFAULT_TRANSPARENCY: f64 = 0.0;
    pub const DEFAULT_SUBSTANCE: Substance = Substance::VACUUM;

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
            transparency: Option::None,
            substance: Option::None,
            refractive_index_override: Option::None,
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
            transparency: self.transparency.unwrap_or(Material::DEFAULT_TRANSPARENCY),
            substance: self.substance.unwrap_or(Material::DEFAULT_SUBSTANCE),
            refractive_index_override: self
                .refractive_index_override
                .unwrap_or(Material::DEFAULT_SUBSTANCE.refractive_index()),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut MaterialBuilder {
        self.color = Option::Some(color);
        self
    }

    pub fn ambient(&mut self, ambient: f64) -> &mut MaterialBuilder {
        self.ambient = Option::Some(ambient);
        self
    }

    pub fn diffuse(&mut self, diffuse: f64) -> &mut MaterialBuilder {
        self.diffuse = Option::Some(diffuse);
        self
    }

    pub fn specular(&mut self, specular: f64) -> &mut MaterialBuilder {
        self.specular = Option::Some(specular);
        self
    }

    pub fn shininess(&mut self, shininess: f64) -> &mut MaterialBuilder {
        self.shininess = Option::Some(shininess);
        self
    }

    pub fn pattern(&mut self, pattern: Pattern) -> &mut MaterialBuilder {
        self.pattern = Option::Some(pattern);
        self
    }

    pub fn reflective(&mut self, reflective: f64) -> &mut MaterialBuilder {
        self.reflective = Option::Some(reflective);
        self
    }

    pub fn transparency(&mut self, transparency: f64) -> &mut MaterialBuilder {
        self.transparency = Option::Some(transparency);
        self
    }

    pub fn substance(&mut self, substance: Substance) -> &mut MaterialBuilder {
        self.substance = Option::Some(substance);
        self
    }

    pub fn refractive_index_override(&mut self, refractive_index: f64) -> &mut MaterialBuilder {
        self.refractive_index_override = Option::Some(refractive_index);
        self
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Substance {
    VACUUM,
    AIR,
    WATER,
    GLASS,
    DIAMOND,
}

impl Substance {
    pub fn refractive_index(&self) -> f64 {
        match self {
            Substance::VACUUM => 1.0,
            Substance::AIR => 1.00029,
            Substance::WATER => 1.333,
            Substance::GLASS => 1.52,
            Substance::DIAMOND => 2.417,
        }
    }
}
