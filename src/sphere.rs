use crate::{
    hittable::{Hit, Hittable},
    point3::Point3,
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
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

        let mut hit = Hit::new();
        hit.p = r.at(root);
        hit.t = root;
        let outward_normal = Vec3::from(hit.p - self.center) / self.radius;
        hit.set_face_normal(r, &outward_normal);

        Some(hit)
    }
}
