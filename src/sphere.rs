use crate::vector::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }

    pub fn to_string(&self) -> String {
        format!("Center: {}\nRadius: {}", self.center.to_string(), self.radius)
    }
}


