use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

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

    pub fn to_string(&self) -> String {
        format!("Center: {}\nRadius: {}", self.center.to_string(), self.radius)
    }
}

impl Hittable for Sphere {
    pub fn hit(&self, ray: &ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        // Determines if a ray hits the sphere.
        //(v•v)t^2 + 2t(v•(A-C)) + (A-C)•(A-C) - r^2 = 0
        // Where v is the ray's directional vector, t is the parameter,
        // A is the origin of the ray, C is the center of the sphere,
        // and r is the radius of the sphere. If the discriminant of the 
        // solution to t is negative there is no intersection.

        let oc: Vec3 = ray.origin() - self.center();
        let a: f32 = Vec3::dot(ray.direction(), ray.direction());
        let b: f32 = 2.0 * Vec3::dot(ray.direction(), oc);
        let c: f32 = Vec3::dot(oc, oc) - self.radius() * self.radius();
        
        let discriminant: f32 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc: f32 = discriminant.sqrt();

        // Find the nearest root in range
        let root: f32 = (-b - sqrt_disc) / (2.0 * a);
        if root < t_min || root > t_max {
            root = (-b + sqrt_disc) / (2.0 * a);
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.origin = ray.at(root);
        
        outward_normal: Vec3 = (hit_record.origin() - self.center()) / radius;
        hit_record.set_face_normal(ray, outward_normal);

        return true;
    }
}


