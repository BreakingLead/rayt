use std::rc::Rc;
use std::time::SystemTime;

use camera::Camera;
use image::RgbImage;
use rand::Rng;

use crate::{hit::HittableList, maths::Color, objects::Plane, objects::Sphere};

mod camera;
mod hit;
mod maths;
mod objects;
mod ray;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 300;
const SAMPLES_PER_PIXEL: u32 = 8;

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
    world.add(Rc::new(Sphere::new([-4.5, -2.5, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([0.0, 2.0, -2.0].into(), 1.0)));
    world.add(Rc::new(Plane::new(
        [-8.0, -2.0, -5.0].into(),
        [16.0, -1.0, 0.0].into(),
        [0.0, 8.0, -10.0].into(),
    )));

    // Render
    println!("Start Rendering...");
    let start_t = SystemTime::now();
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let y = IMAGE_HEIGHT - y - 1;
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _i in 0..SAMPLES_PER_PIXEL {
            let ray = camera.get_ray(
                IMAGE_WIDTH,
                IMAGE_HEIGHT,
                x as f64 + rng.gen::<f64>(),
                y as f64 + rng.gen::<f64>(),
            );
            pixel_color = pixel_color + ray.get_pixel_color(&world);
        }
        pixel_color = pixel_color / SAMPLES_PER_PIXEL as f64;
        *pixel = pixel_color.into();
    }
    let render_time = start_t.elapsed().unwrap().as_millis() as f64 / 1000.0;

    println!("Elasped {} sec.", render_time);
    img.save("test.png").unwrap();
}
