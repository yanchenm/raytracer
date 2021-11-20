use std::{fs::File, io::BufWriter};
use std::io::Write;

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

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            let pixel = format!("{} {} {}\n", ir, ig, ib);
            file_buffer.write(pixel.as_bytes()).unwrap();
        }
    }

    file_buffer.flush().unwrap();
}
