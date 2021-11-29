use std::{
    fs::File,
    io::{BufWriter, Write},
};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    pub channels: Vec3,
}

impl_op_ex!(+ |a: Colour, b: Colour| -> Colour {
    Colour {
        channels: a.channels + b.channels,
    }
});

impl_op_ex!(+= |a: &mut Colour, b: Colour| {
    a.channels += b.channels
});

impl_op_ex_commutative!(+ |c: Colour, v: Vec3| -> Colour {
    Colour {
        channels: c.channels + v
    }
});

impl_op_ex_commutative!(*|c: Colour, f: f64| -> Colour {
    Colour {
        channels: c.channels * f,
    }
});

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour {
            channels: Vec3::new(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        self.channels[0]
    }

    pub fn g(&self) -> f64 {
        self.channels[1]
    }

    pub fn b(&self) -> f64 {
        self.channels[2]
    }

    pub fn pixel_string(&self) -> String {
        let r = (256.0 * self.r().clamp(0.0, 0.999)) as i64;
        let g = (256.0 * self.g().clamp(0.0, 0.999)) as i64;
        let b = (256.0 * self.b().clamp(0.0, 0.999)) as i64;

        format!("{} {} {}\n", r, g, b)
    }

    pub fn write_colour(&self, file: &mut BufWriter<File>, samples_per_pixel: i64) {
        // Average colour over number of samples and gamma correct for gamma = 2.0
        let scale = 1.0 / samples_per_pixel as f64;
        let scaled_colour = Colour {
            channels: Vec3 {
                x: (self.r() * scale).sqrt(),
                y: (self.g() * scale).sqrt(),
                z: (self.b() * scale).sqrt(),
            },
        };

        file.write(scaled_colour.pixel_string().as_bytes()).unwrap();
    }
}
