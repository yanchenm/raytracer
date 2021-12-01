use crate::{
    colour::Colour,
    hittable::Hit,
    material::{Material, Scatter},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Colour, fuzz: f64) -> Self {
        Metal {
            albedo: *albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray_in.direction().unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter {
            scattered,
            attenuation: self.albedo,
        })
    }
}
