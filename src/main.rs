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
use util::{INFINITY, ASPECT_RATIO, WIDTH, HEIGHT, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, SAMPLES, random_f32, ORIGIN};

extern crate image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};


pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut record: HitRecord = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return 255.0 * 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = ray.direction().normalize();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    
    255.0 * ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}


fn main() {

    // Image

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.4)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(list);

    // Camera
    let cam: Camera = Camera::new();

    // Render image
    let mut output = DynamicImage::new_rgb8(WIDTH as u32, HEIGHT as u32);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES {
                let u = (x as f32 + random_f32()) / (WIDTH - 1) as f32;
                let v = ((HEIGHT - y) as f32 + random_f32()) / (HEIGHT - 1) as f32;
                let ray: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            // Divide the color by the number of samples
            pixel_color /= SAMPLES as f32;

            output.put_pixel(x as u32, y as u32, Rgba::from_channels(
                pixel_color.x() as u8,
                pixel_color.y() as u8,
                pixel_color.z() as u8,
                255)
            );
        }
    }

    DynamicImage::save(&output, &Path::new("images/test.png"));

}
