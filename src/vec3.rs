use std::ops;

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl_op_ex!(-|v: Vec3| -> Vec3 { Vec3::new(-v.x, -v.y, -v.z) });

impl_op_ex!(+ |u: Vec3, v: Vec3| -> Vec3 { Vec3::new(u.x + v.x, u.y + v.y, u.z + v.z) });
impl_op_ex!(+= |u: &mut Vec3, v: Vec3| { u.x += v.x; u.y += v.y; u.z += v.z; });

impl_op_ex!(-|u: Vec3, v: Vec3| -> Vec3 { Vec3::new(u.x - v.x, u.y - v.y, u.z - v.z) });
impl_op_ex!(-= |u: &mut Vec3, v: Vec3| { u.x -= v.x; u.y -= v.y; u.z -= v.z; });

// Element-wise multiplication
impl_op_ex!(*|u: Vec3, v: Vec3| -> Vec3 { Vec3::new(u.x * v.x, u.y * v.y, u.z * v.z) });

impl_op_ex_commutative!(*|v: Vec3, c: f64| -> Vec3 { Vec3::new(v.x * c, v.y * c, v.z * c) });
impl_op_ex!(*= |v: &mut Vec3, c: f64| { v.x *= c; v.y *= c; v.z *= c; });

impl_op_ex!(/ |v: Vec3, c: f64| -> Vec3 { Vec3::new(v.x / c, v.y / c, v.z / c) });
impl_op_ex!(/= |v: &mut Vec3, c: f64| { v.x /= c; v.y /= c; v.z /= c; });

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
}
