use std::io::Write;
use std::{fs::File, io::BufWriter};

use indicatif::ProgressBar;

use crate::colour::Colour;

mod colour;
mod point3;
mod ray;
mod vec3;

fn main() {
    // Image dimensions
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // File I/O
    let file = File::create("output/image.ppm").expect("Unable to create file.");
    let mut file_buffer = BufWriter::new(file);

    // Write file header
    let image_header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    file_buffer.write(image_header.as_bytes()).unwrap();

    let bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel = Colour::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            file_buffer.write(pixel.pixel_string().as_bytes()).unwrap();
            bar.inc(1);
        }
    }

    file_buffer.flush().unwrap();
    bar.finish();
}
