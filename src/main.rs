use std::rc::Rc;

use camera::Camera;
use image::RgbImage;

use crate::{hit::HittableList, maths::Vec3, objects::Sphere};

mod camera;
mod hit;
mod maths;
mod objects;
mod ray;
mod utils;

const IMAGE_WIDTH: u32 = 320;
const IMAGE_HEIGHT: u32 = 160;

fn main() {
    // Image

    // Open File
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::new();

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new([3.0, 1.0, -3.0].into(), 1.0)));

    // Render
    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     *pixel = ray.get_pixel_color(&world).into();
    // }

    for y in (0..img.height()).rev() {
        for x in 0..img.width() {
            let ray = camera.get_ray(img.width(), img.height(), x, y);

            img.put_pixel(
                x,
                y,
                ray.get_pixel_color(&Sphere::new([1.0, 1.5, -2.0].into(), 1.0))
                    .into(),
            );
        }
    }
    println!("Done");
    img.save("test2.png").unwrap();
}
