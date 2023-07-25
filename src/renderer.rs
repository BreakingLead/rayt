use std::f64::INFINITY;
use std::time::SystemTime;
use image::{RgbImage, ImageBuffer, Rgb};
use rand::Rng;

use crate::{camera::Camera, hit::{HittableList, Hittable}, light::{Light, LightGroup}, maths::Color, ray::Ray};
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL};

pub enum Shader {
    Phong,
    BlinnPhong,
    WhittedStyleRayTracing,
    PathTracing,
}

pub struct Renderer {
    pub world: HittableList,
    pub light_group: LightGroup,
    pub camera: Camera,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(world: HittableList, light_group: LightGroup, camera: Camera, shader: Shader) -> Self {
        Self { world, light_group, camera, shader }
    }

    fn get_pixel_color(&self, ray: &Ray) -> Color {
        match self.world.get_hit_record(ray, 0.0, INFINITY) {
            Some(record) => {
                match self.shader {
                    Shader::Phong => {
                        //todo
                        Color::new(1.0, 1.0, 1.0)
                    }
                    Shader::BlinnPhong => {
                        //todo
                        Color::new(1.0, 1.0, 1.0)
                    }
                    Shader::WhittedStyleRayTracing => {
                        //todo
                        Color::new(1.0, 1.0, 1.0)
                    }
                    Shader::PathTracing => {
                        //todo
                        Color::new(1.0, 1.0, 1.0)
                    }
                }
            }
            None => {
                Color::new(0.0, 0.0, 0.0)
            }
        }
    }

    pub fn render(&self, output: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
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

        img
    }
}