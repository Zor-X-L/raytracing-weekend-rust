use crate::float::Float;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord { p: Point3::zero(), normal: Vec3::zero(), t: 0.0, front_face: false }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool;
}