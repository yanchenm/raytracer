use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: Point3 {
                position: origin
                    - horizontal / 2.0
                    - vertical / 2.0
                    - Vec3::new(0.0, 0.0, focal_length),
            },
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            Vec3::from(self.lower_left_corner) + (u * self.horizontal) + (v * self.vertical)
                - self.origin,
        )
    }
}
