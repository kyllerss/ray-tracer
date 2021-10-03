use crate::domain::light::Light;
use crate::domain::object::Sphere;

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_source: Option<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light_source: Option::None,
        }
    }

    pub fn add_object(&mut self, obj: Sphere) -> &Self {
        self.objects.push(obj);
        self
    }
}
