use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::f64::INFINITY;
use std::time::SystemTime;

use crate::const_vars::ConstContext;
use crate::{
    camera::Camera,
    hit::{Hittable, HittableList},
    light::Light,
    maths::Color,
    ray::Ray,
};

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
    pub ctx: ConstContext,
}

impl Renderer {
    pub fn new(
        world: HittableList,
        light: Light,
        camera: Camera,
        shader: Shader,
        ctx: ConstContext,
    ) -> Self {
        Self {
            world,
            light,
            camera,
            shader,
            ctx,
        }
    }

    fn get_pixel_color(&self, ray: &Ray) -> Color {
        match self.world.get_hit_record(ray, 0.0, INFINITY) {
            Some(record) => {
                // TODO: Shader for this place
                let normal = record.normal;

                Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
            None => {
                let y_ratio = (ray.direction.normalize().y + 1.0) / 2.0;

                // Linear interpolation method
                Color::new(1.0, 1.0, 1.0) * (1.0 - y_ratio) + Color::new(0.5, 0.5, 0.7) * y_ratio
            }
        }
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(self.camera.image_width, self.camera.image_height);

        let start_t = SystemTime::now();
        let mut rng = rand::thread_rng();

        let bar = ProgressBar::new((self.camera.image_height * self.camera.image_width).into())
            .with_style(
                ProgressStyle::with_template(
                    "{spinner:.green}  [{percent:.}%] [{elapsed_precise}] [{bar:60.cyan/blue}] {pos:>7.green}/{len:7.bold} {msg:>}",
                )
                .unwrap()
                .progress_chars("#>-"),
            );

        // Draw pixels
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let y = self.camera.image_height as u32 - y - 1;
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for i in 0..self.ctx.samples_per_pixel {
                let ray = self.camera.get_ray(
                    self.camera.image_width,
                    self.camera.image_height,
                    x as f64 + rng.gen::<f64>(),
                    y as f64 + rng.gen::<f64>(),
                );

                pixel_color = pixel_color + self.get_pixel_color(&ray);
            }
            pixel_color = pixel_color / self.ctx.samples_per_pixel as f64;

            *pixel = pixel_color.into();
            bar.inc(1);
        }
        bar.finish_and_clear();

        let render_time = start_t.elapsed().unwrap().as_millis() as f64 / 1000.0;

        if self.ctx.output {
            println!("Time elapsed: {}", render_time);
        }

        img
    }
}
