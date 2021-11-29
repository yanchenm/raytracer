use std::f64::INFINITY;
use std::io::Write;
use std::{fs::File, io::BufWriter};

use indicatif::ProgressBar;

use colour::Colour;
use hittable::Hittable;
use hittable_list::HittableList;
use point3::Point3;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

mod colour;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &impl Hittable) -> Colour {
    let hit_res = world.hit(ray, 0.0, INFINITY);
    match hit_res {
        Some(hit) => {
            return 0.5 * (hit.normal + Colour::new(1.0, 1.0, 1.0));
        }
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // File I/O
    let file =
        File::create("output/coloured_sphere_with_ground.ppm").expect("Unable to create file.");
    let mut file_buffer = BufWriter::new(file);

    // Write file header
    let image_header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file_buffer.write(image_header.as_bytes()).unwrap();

    // World
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    let mut world = HittableList::new();
    world.add(&sphere);
    world.add(&ground);

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
            let pixel_color = ray_color(&r, &world);

            file_buffer
                .write(pixel_color.pixel_string().as_bytes())
                .unwrap();

            bar.inc(1);
        }
    }

    file_buffer.flush().unwrap();
    bar.finish();
}
