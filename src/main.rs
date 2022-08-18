use crate::camera::Camera;
use crate::color::{Color, write_color};
use crate::float::Float;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::rand::Xoshiro256PlusPlus;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, random_double01};
use crate::sphere::Sphere;
use crate::vec3::{Point3, unit_vector};

mod camera;
mod color;
mod float;
mod hittable;
mod hittable_list;
mod rand;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord::empty();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rand = Xoshiro256PlusPlus::default();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as Float / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World

    let mut world = HittableList::empty();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::default();

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as Float + random_double01(&mut rand)) / (image_width - 1) as Float;
                let v = (j as Float + random_double01(&mut rand)) / (image_height - 1) as Float;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel)
                .expect("Error");
        }
    }

    eprint!("\nDone.\n");
}