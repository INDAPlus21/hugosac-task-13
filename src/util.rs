use rand::{Rng};


pub const INFINITY: f32 = std::f32::INFINITY;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: i16 = 1024;
pub const HEIGHT: i16 = (WIDTH as f32 / ASPECT_RATIO) as i16;
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const SAMPLES: i16 = 100;
pub const MAX_DEPTH: i8 = 50;


// Return a random float in [0, 1)
pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

// Return a random float in [min, max)
pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
