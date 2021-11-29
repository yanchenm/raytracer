use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    // Returns a hit record if ray hits objects, else None.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
