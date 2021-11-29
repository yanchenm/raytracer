use crate::{
    colour::Colour,
    hittable::Hit,
    material::{Material, Scatter},
    ray::Ray,
};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Colour,
}

impl Metal {
    pub fn new(albedo: &Colour) -> Self {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray_in.direction().unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected);

        if scattered.direction().dot(hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter {
            scattered,
            attenuation: self.albedo,
        })
    }
}
