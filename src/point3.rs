use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub location: Vec3,
}

impl_op_ex_commutative!(+ |p: Point3, v: Vec3| -> Point3 { Point3 { location: p.location + v } });
impl_op_ex!(-|p: Point3, v: Vec3| -> Point3 {
    Point3 {
        location: p.location - v,
    }
});
impl_op_ex!(-|p: Point3, q: Point3| -> Point3 {
    Point3 {
        location: p.location - q.location,
    }
});

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
