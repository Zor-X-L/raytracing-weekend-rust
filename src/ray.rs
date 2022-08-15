use crate::Float;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray { orig: Point3::zero(), dir: Vec3::zero() }
    }
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: Float) -> Vec3 {
        self.orig + t * self.dir
    }
}