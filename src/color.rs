use std::io;
use std::io::Write;

use crate::vec3::Vec3;

// RGB color
pub type Color = Vec3;

pub fn write_color(write: &mut impl Write, pixel_color: Color) -> io::Result<()> {
    write!(write, "{} {} {}\n",
           (255.999 * pixel_color.x()) as u8,
           (255.999 * pixel_color.y()) as u8,
           (255.999 * pixel_color.z()) as u8
    )?;
    Ok(())
}