use std::path::Path;

mod vector;
mod sphere;
mod ray;
mod hit;
mod camera;
mod util;

use vector::Vec3;
use sphere::Sphere;
use ray::Ray;
use hit::{Hittable, HittableList, HitRecord};
use camera::Camera;
use util::{INFINITY, WIDTH, HEIGHT, SAMPLES, MAX_DEPTH, random_f32};

extern crate image;
use image::{DynamicImage, GenericImage, Pixel, Rgb, Rgba, GenericImageView, ImageBuffer, RgbImage};


pub fn ray_color(ray: &Ray, world: &HittableList, depth: i8) -> Vec3 {
    let mut record: HitRecord = HitRecord::new();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.0, INFINITY, &mut record) {
        let target: Vec3 = record.origin + record.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(record.origin, target - record.origin), world, depth - 1)
    }

    let unit_direction: Vec3 = ray.direction().normalize();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    
    255.0 * ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}


fn main() {

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.4)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(list);

    // Camera
    let cam: Camera = Camera::new();

    // Render image

    let mut img = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES {
                let u = (x as f32 + random_f32()) / (WIDTH - 1) as f32;
                let v = ((HEIGHT - y) as f32 + random_f32()) / (HEIGHT - 1) as f32;
                let ray: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            // Divide the color by the number of samples
            pixel_color /= SAMPLES as f32;

            img.put_pixel(x as u32, y as u32, Rgb::from_channels(
                pixel_color.x() as u8,
                pixel_color.y() as u8,
                pixel_color.z() as u8, 
                0
            ));
        }
    }

    ImageBuffer::save(&img, &Path::new("images/image.png"));
}
