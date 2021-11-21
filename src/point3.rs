use std::ops;

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    location: Vec3,
}

impl ops::Add<&Vec3> for &Point3 {
    type Output = Point3;
    fn add(self, other: &Vec3) -> Point3 {
        Point3 {
            location: &self.location + other,
        }
    }
}

impl ops::Sub<&Vec3> for &Point3 {
    type Output = Point3;
    fn sub(self, other: &Vec3) -> Point3 {
        Point3 {
            location: &self.location - other,
        }
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            location: Vec3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f64 {
        self.location.x
    }

    pub fn y(&self) -> f64 {
        self.location.y
    }

    pub fn z(&self) -> f64 {
        self.location.z
    }
}
