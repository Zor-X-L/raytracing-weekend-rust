use crate::float::Float;
use crate::rand::Rand;

// Constants

pub const INFINITY: Float = Float::INFINITY;
pub const PI: Float = std::f64::consts::PI as Float;

// Utility Functions

pub fn degrees_to_radians(degrees: Float) -> Float {
    degrees * PI / 180.0
}

pub fn random_double01(rand: &mut dyn Rand) -> Float {
    rand.rand() as Float / (rand.rand_max() as Float + 1.0)
}

pub fn random_double(rand: &mut dyn Rand, min: Float, max: Float) -> Float {
    min + (max - min) * random_double01(rand)
}

pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}