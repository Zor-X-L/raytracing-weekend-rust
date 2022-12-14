use std::io;
use std::io::Write;

use crate::float::Float;
use crate::rtweekend::clamp;
use crate::vec3::Vec3;

// RGB color
pub type Color = Vec3;

pub fn write_color(write: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as Float;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    write!(write, "{} {} {}\n",
           (256.0 * clamp(r, 0.0, 0.999)) as i32,
           (256.0 * clamp(g, 0.0, 0.999)) as i32,
           (256.0 * clamp(b, 0.0, 0.999)) as i32
    )?;
    Ok(())
}