use std::rc::Rc;

use camera::Camera;
use image::RgbImage;
use rand::{random, Rng};

use crate::{
    hit::HittableList,
    maths::{Color, Vec3},
    objects::{Ground, Sphere},
};

mod camera;
mod hit;
mod maths;
mod objects;
mod ray;
mod utils;

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 150;
const SAMPLES_PER_PIXEL: u32 = 15;

fn main() {
    // Image

    // Open File
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::new();

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new([3.0, 1.0, -10.0].into(), 1.0)));
    world.add(Rc::new(Sphere::new([-3.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-6.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-4.5, 2.5, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([0.0, 2.0, -2.0].into(), 1.0)));

    // Render
    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     *pixel = ray.get_pixel_color(&world).into();
    // }
    let mut rng = rand::thread_rng();
    for y in (0..img.height()).rev() {
        for x in 0..img.width() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for i in (0..SAMPLES_PER_PIXEL) {
                let ray = camera.get_ray(
                    img.width(),
                    img.height(),
                    x as f64 + rng.gen::<f64>(),
                    y as f64 + rng.gen::<f64>(),
                );

                pixel_color = pixel_color + ray.get_pixel_color(&world);
            }
            pixel_color = pixel_color / SAMPLES_PER_PIXEL as f64;

            img.put_pixel(x, img.height() - y - 1, pixel_color.into());
        }
    }
    println!("Done");
    img.save("test2.png").unwrap();
}
