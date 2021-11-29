use crate::{
    hittable::{Hit, Hittable},
    material::Material,
    point3::Point3,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a impl Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = Vec3::from(r.origin() - self.center);

        // Coefficients of quadratic equation
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit = Hit {
            p: r.at(root),
            t: root,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: self.material,
        };

        let outward_normal = Vec3::from(hit.p - self.center) / self.radius;
        hit.set_face_normal(r, &outward_normal);

        Some(hit)
    }
}
