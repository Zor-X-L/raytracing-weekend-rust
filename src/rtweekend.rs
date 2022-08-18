use crate::float::Float;

// Constants

pub const INFINITY: Float = Float::INFINITY;
pub const PI: Float = std::f64::consts::PI as Float;

// Utility Functions

pub fn degrees_to_radians(degrees: Float) -> Float {
    degrees * PI / 180.0
}