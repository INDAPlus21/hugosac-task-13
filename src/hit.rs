use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub origin: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            origin: Vec3::new(0.0,0.0,0.0),
            normal: Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false
        }
    }

}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list: list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;

        for object in &self.list {
            if object.hit(&ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}