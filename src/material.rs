use crate::color::Color;
use crate::hittable::HitRecord;
use crate::rand::Rand;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rand: &mut dyn Rand) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rand: &mut dyn Rand) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector(rand);

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        scattered.clone_from(&Ray::new(rec.p, scatter_direction));
        attenuation.clone_from(&self.albedo);
        true
    }
}