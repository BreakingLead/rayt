use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::time::SystemTime;

use crate::{
    camera::Camera,
    const_vars::ConstContext,
    hit::HittableList,
    light::LightGroup,
    maths::Color,
    ray::Ray,
    shaders::ShaderType,
};

pub struct Renderer {
    pub world: HittableList,
    pub light_group: LightGroup,
    pub camera: Camera,
    pub ctx: ConstContext,
    pub shader_type: ShaderType,
    pub gamma: f64,
}

impl Renderer {
    pub fn new(
        world: HittableList,
        light_group: LightGroup,
        camera: Camera,
        ctx: ConstContext,
        shader_type: ShaderType,
        gamma: f64,
    ) -> Self {
        Self {
            world,
            light_group,
            camera,
            ctx,
            shader_type,
            gamma,
        }
    }

    pub fn get_pixel_color(&self, ray: &Ray) -> Color {
        match self.shader_type {
            ShaderType::PathTracing => self.shader_path_tracing(ray, self.ctx.max_depth as i32)
                                            .gamma_correction(self.gamma),
            ShaderType::
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
