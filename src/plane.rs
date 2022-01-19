use crate::vector::Vec3;

pub struct Plane {
    point: Vec3,
    normal: Vec3
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Plane {
        Plane {
            point: point,
            normal: normal
        }
    }
}