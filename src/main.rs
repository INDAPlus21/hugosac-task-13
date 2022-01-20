use std::path::Path;

mod vector;
mod sphere;
mod ray;
mod hit;
mod util;

use vector::Vec3;
use sphere::Sphere;
use ray::Ray;
use hit::{Hittable, HittableList, HitRecord};
use util::{INFINITY};

extern crate image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};


pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut record: HitRecord = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return 255.0 * 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = ray.direction().normalize();
    let t2: f32 = 0.5 * (unit_direction.y() + 1.0);
    
    255.0 * ((1.0 - t2) * Vec3::new(1.0, 1.0, 1.0) + t2 * Vec3::new(0.5, 0.7, 1.0))
}


fn main() {

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width: i16 = 475;
    let height: i16 = (width as f32 / aspect_ratio) as i16;

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.4)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(list);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    // let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.4);

    // Render image
    let mut output = DynamicImage::new_rgb8(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let v = (height - y) as f32 / (height - 1) as f32;
            let ray: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: Vec3 = ray_color(&ray, &world);

            output.put_pixel(x as u32, y as u32, Rgba::from_channels(
                pixel_color.x() as u8,
                pixel_color.y() as u8,
                pixel_color.z() as u8,
                255)
            );
        }
    }

    DynamicImage::save(&output, &Path::new("images/test_sphere.png"));

}
