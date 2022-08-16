use std::fmt;
use std::fmt::Formatter;
use std::ops;

use crate::float::Float;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e0: Float,
    pub e1: Float,
    pub e2: Float,
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 }
    }
    pub fn new(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3 { e0, e1, e2 }
    }

    pub fn x(&self) -> Float { self.e0 }
    pub fn y(&self) -> Float { self.e1 }
    pub fn z(&self) -> Float { self.e2 }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Float {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e0, -self.e1, -self.e2)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!()
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => panic!()
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e0 += rhs.e0;
        self.e1 += rhs.e1;
        self.e2 += rhs.e2;
    }
}

impl ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        *self *= 1. / rhs;
    }
}

// Type aliases for Vec3

// 3D point
pub type Point3 = Vec3;

// Vec3 Utility Functions

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.e0 + rhs.e0, self.e1 + rhs.e1, self.e2 + rhs.e2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.e0 - rhs.e0, self.e1 - rhs.e1, self.e2 - rhs.e2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.e0 * rhs.e0, self.e1 * rhs.e1, self.e2 * rhs.e2)
    }
}

impl ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.e0, self * rhs.e1, self * rhs.e2)
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub fn dot(u: Vec3, v: Vec3) -> Float {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e1 * v.e2 - u.e2 * v.e1,
        u.e2 * v.e0 - u.e0 * v.e2,
        u.e0 * v.e1 - u.e1 * v.e0,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}