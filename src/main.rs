use camera::Camera;
use image::RgbImage;

use crate::{maths::Vec3, objects::Sphere};

mod camera;
mod maths;
mod hit;
mod objects;
mod ray;

const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = 900;

fn main() {
    // Image

    // Open File
    let mut img_buf = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::new();

    // Render
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let x_ratio = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let y_ratio = y as f64 / (IMAGE_HEIGHT - 1) as f64;

        let ray = camera.get_ray(x_ratio, y_ratio);

        let sphere = Sphere::new(Vec3::new(0.3, 0.3, -1.0), 0.5);

        *pixel = ray.result(&sphere).into();
    }
    println!("Done");
    img_buf.save("test.png").unwrap();
}
