use std::f64::INFINITY;
use std::time::SystemTime;
use image::{RgbImage, ImageBuffer, Rgb};
use rand::Rng;

use crate::{camera::Camera, hit::{HittableList, Hittable}, light::Light, maths::Color, ray::Ray};
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL};

pub enum Shader {
    Phong,
    BlinnPhong,
    WhittedStyleRayTracing,
    PathTracing,
}

pub struct Renderer {
    pub world: HittableList,
    pub light: Light,
    pub camera: Camera,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(world: HittableList, light: Light, camera: Camera, shader: Shader) -> Self {
        let res = Self {
            world,
            light,
            camera,
            shader,
        };

        res
    }

    fn get_pixel_color(&self, ray: &Ray) -> Color {
        match self.world.get_hit_record(ray, 0.0, INFINITY) {
            Some(record) => {
                let normal = record.normal;

                Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
            None => {
                let y_ratio = (ray.direction.normalize().y + 1.0) / 2.0;

                Color::new(1.0, 1.0, 1.0) * (1.0 - y_ratio) + Color::new(0.5, 0.5, 0.7) * y_ratio
            }
        }
    }

    pub fn render(&self, output: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        let start_t = SystemTime::now();
        let mut rng = rand::thread_rng();

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if output {
                //todo: output rendering time & process
            }

            let y = IMAGE_HEIGHT - y - 1;

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for i in 0..SAMPLES_PER_PIXEL {
                let ray = self.camera.get_ray(
                    IMAGE_WIDTH,
                    IMAGE_HEIGHT,
                    x as f64 + rng.gen::<f64>(),
                    y as f64 + rng.gen::<f64>(),
                );

                pixel_color = pixel_color + self.get_pixel_color(&ray);
            }
            pixel_color = pixel_color / SAMPLES_PER_PIXEL as f64;

            *pixel = pixel_color.into();
        }
        let render_time = start_t.elapsed().unwrap().as_millis() as f64 / 1000.0;

        img
    }
}