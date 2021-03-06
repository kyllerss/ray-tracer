use crate::domain::color::Color;
use crate::domain::material::Material;
use crate::domain::object::Object;
use crate::domain::{Point, Vector};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    // constructor
    pub fn new(position: Point, intensity: Color) -> Light {
        Light {
            position,
            intensity,
        }
    }

    // TODO Should move this over to Sphere. Perhaps turn this into a traight that Sphere and others implement.
    pub fn lighting(
        material: &Material,
        object: &Object,
        light: &Light,
        point: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match &material.pattern {
            Some(pattern) => pattern.color_at(object, point),
            None => material.color,
        };

        let effective_color = &color * &light.intensity;
        let light_v = (&light.position - point).normalize();
        let ambient = &effective_color * material.ambient as f32;
        let light_dot_normal = light_v.dot_product(normal_v);

        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = Color::BLACK;
            specular = Color::BLACK;
        } else {
            // calculate diffuse
            diffuse = &effective_color * (material.diffuse as f32 * light_dot_normal as f32);

            // calculate specular
            let reflect_v = (-light_v).reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot_product(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Color::BLACK;
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = &light.intensity * (material.specular as f32 * factor as f32);
            }
        }

        // return color calculation
        if in_shadow {
            ambient
        } else {
            &(&ambient + &diffuse) + &specular
        }
    }
}
