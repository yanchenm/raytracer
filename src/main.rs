use std::io::Write;
use std::{fs::File, io::BufWriter};

use indicatif::ProgressBar;

use crate::colour::Colour;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod colour;
mod point3;
mod ray;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = Vec3::from(r.origin() - center);

    // Coefficients of quadratic equation
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - (radius * radius);

    let discriminant = (half_b * half_b) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray) -> Colour {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let N = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Colour::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // File I/O
    let file = File::create("output/coloured_sphere.ppm").expect("Unable to create file.");
    let mut file_buffer = BufWriter::new(file);

    // Write file header
    let image_header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file_buffer.write(image_header.as_bytes()).unwrap();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let bar = ProgressBar::new((image_width * image_height) as u64);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            file_buffer
                .write(pixel_color.pixel_string().as_bytes())
                .unwrap();

            bar.inc(1);
        }
    }

    file_buffer.flush().unwrap();
    bar.finish();
}
