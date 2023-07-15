use std::{fs, io::Write};

use camera::Camera;
use maths::Color;

use crate::{maths::Vec3, objects::Sphere, ray::Ray};

mod camera;
mod maths;
mod objects;
mod ray;

const IMAGE_WIDTH: usize = 1600;
const IMAGE_HEIGHT: usize = 900;

fn main() {
    // Image

    // Open File
    let mut img = fs::File::create("test.ppm").expect("Create failed");
    let mut buf: Vec<u8> = vec![];

    // let mut window = Window::new(
    //     "RayTracer",
    //     IMAGE_WIDTH,
    //     IMAGE_HEIGHT,
    //     WindowOptions::default(),
    // )
    // .expect("Failed");

    // Camera
    let camera = Camera::new();

    // Render
    write!(&mut buf, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("Failed");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("{} lines remaining...", j);

        for i in 0..IMAGE_WIDTH {
            let x_ratio = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let y_ratio = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = camera.get_ray(x_ratio, y_ratio);

            let sp = Sphere::new(Vec3::new(0.2, 0.3, -2.0), 0.5);

            let mut write_color = |c: Ray| write!(&mut buf, "{}", c.result(&sp)).expect("Failed");

            write_color(ray);
        }
    }

    img.write_all(&buf).expect("Can't write");
}
