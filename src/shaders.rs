use std::{f64::INFINITY, ops::Deref};

use crate::{hit::{HitRecord, Hittable}, maths::{Color, Vec3, HDR}, renderer::Renderer, ray::Ray};
use crate::bsdf::BSDF;

pub enum ShaderType {
    PathTracing,
}

impl Renderer {
    pub fn shader_path_tracing(&self, ray: &Ray, depth: i32) -> HDR {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match self.world.get_hit_record(&ray, 0.0, INFINITY) {
            Some(record) => {
                let rand_ray = Ray::new(record.point, Vec3::rand_hemisphere_dir(record.normal));
                return self.shader_path_tracing(&rand_ray, depth - 1) * 0.5;
            }
            None => {
                return Color::new(1.0, 1.0, 1.0);
            }
        }
    }
}
