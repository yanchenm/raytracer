use crate::{colour::Colour, hittable::Hit, point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Colour,
}

impl Scatter {
    pub fn new() -> Self {
        Scatter {
            scattered: Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            attenuation: Colour::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Material {
    // Returns a scattered ray and how much the ray should be attenuated
    // or None if the ray was absorbed.
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}
