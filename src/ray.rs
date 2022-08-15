use crate::Float;
use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    fn zero() -> Ray {
        Ray { orig: Point3::zero(), dir: Vec3::zero() }
    }
    fn new(origin: &Point3, direction: &Vec3) -> Ray {
        Ray { orig: *origin, dir: *direction }
    }

    fn origin(&self) -> Point3 {
        self.orig
    }
    fn direction(&self) -> Vec3 {
        self.dir
    }

    fn at(&self, t: Float) -> Vec3 {
        &self.orig + &(t * &self.dir)
    }
}