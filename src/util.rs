use rand::{Rng};

pub const INFINITY: f32 = std::f32::INFINITY;

// Returna random float in [0, 1)
pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

// Returna random float in [min, max)
pub fn random_in_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}