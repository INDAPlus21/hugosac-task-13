use crate::vector::Vec3;
use crate::ray::Ray;

use crate::util::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let focal_length = 1.0;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}