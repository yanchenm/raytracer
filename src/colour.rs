use crate::vec3::Vec3;

pub struct Colour {
    channels: Vec3,
}

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
        let r = (self.r() * 255.999) as i32;
        let g = (self.g() * 255.999) as i32;
        let b = (self.b() * 255.999) as i32;

        format!("{} {} {}\n", r, g, b)
    }
}
