use crate::{
    colour::Colour,
    hittable::Hit,
    material::{Material, Scatter},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: &Colour) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        // Catch the degenerate scatter direction (random unit vector was opposite normal)
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Scatter {
            scattered: Ray::new(hit.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
