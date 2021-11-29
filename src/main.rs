use std::f64::INFINITY;
use std::io::Write;
use std::{fs::File, io::BufWriter};

use indicatif::ProgressBar;
use rand::Rng;

use camera::Camera;
use colour::Colour;
use hittable::Hittable;
use hittable_list::HittableList;
use point3::Point3;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;
mod vec3;

fn ray_colour(ray: &Ray, world: &impl Hittable, depth: i64) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let hit_res = world.hit(ray, 0.001, INFINITY);
    match hit_res {
        Some(hit) => {
            let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
            return 0.5
                * ray_colour(
                    &Ray::new(hit.p, Vec3::from(target - hit.p)),
                    world,
                    depth - 1,
                );
        }
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // File I/O
    let file = File::create("output/07-diffuse_sphere_gamma_corrected.ppm")
        .expect("Unable to create file.");
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
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    // Render
    let bar = ProgressBar::new((image_width * image_height) as u64);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);

                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world, max_depth);
            }
            pixel_colour.write_colour(&mut file_buffer, samples_per_pixel);
            bar.inc(1);
        }
    }

    file_buffer.flush().unwrap();
    bar.finish();
}
