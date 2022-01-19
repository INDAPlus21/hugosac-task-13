use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
}