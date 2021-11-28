use auto_ops::{impl_op, impl_op_ex, impl_op_ex_commutative};

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub position: Vec3,
}

impl_op_ex_commutative!(+ |p: Point3, v: Vec3| -> Point3 { Point3 { position: p.position + v } });
impl_op_ex!(-|p: Point3, v: Vec3| -> Vec3 { p.position - v });
impl_op_ex!(-|p: Point3, q: Point3| -> Point3 {
    Point3 {
        position: p.position - q.position,
    }
});
impl_op_ex!(-|v: Vec3, p: Point3| -> Vec3 { v - p.position });
impl_op!(-|p: Point3, q: &Point3| -> Point3 {
    Point3 {
        position: p.position - q.position,
    }
});

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            position: Vec3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x
    }

    pub fn y(&self) -> f64 {
        self.position.y
    }

    pub fn z(&self) -> f64 {
        self.position.z
    }
}
