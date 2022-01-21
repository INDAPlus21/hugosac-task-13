use rand::{Rng};

use crate::vector::Vec3;

pub const INFINITY: f32 = std::f32::INFINITY;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: i16 = 1280;
pub const HEIGHT: i16 = (WIDTH as f32 / ASPECT_RATIO) as i16;
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const SAMPLES: i16 = 100;

pub const ORIGIN: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};

// Returna random float in [0, 1)
pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

// Return a random float in [min, max)
pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min };
    if x > max { return max };
    return x;
}