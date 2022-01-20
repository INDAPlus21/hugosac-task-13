use crate::vector::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub origin: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {


    pub fn set_face_normal(&self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}


pub trait Hittable {
    pub fn hit(&self, ray: &ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}