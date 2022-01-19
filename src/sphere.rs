use crate::vector::Vec3;
use crate::ray::Ray;

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

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc: Vec3 = ray.origin() - self.center();
        let a: f32 = Vec3::dot(ray.direction(), ray.direction());
        let b: f32 = 2.0 * Vec3::dot(oc, ray.direction());
        let c: f32 = Vec3::dot(oc, oc) - self.radius() * self.radius();
        let discriminant: f32 = b * b - 4.0 * a * c;
        
        discriminant > 0.0
    }

    pub fn to_string(&self) -> String {
        format!("Center: {}\nRadius: {}", self.center.to_string(), self.radius)
    }
}


