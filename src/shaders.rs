use std::{f64::INFINITY, ops::Deref};

use crate::bsdf::BSDF;
use crate::{
    hit::{HitRecord, Hittable},
    maths::{Color, Vec3, HDR},
    ray::Ray,
    renderer::Renderer,
};

pub enum ShaderType {
    PathTracing,
    LeadTest,
}

impl Renderer {
    pub fn shader_path_tracing(&self, ray: &Ray, depth: i32) -> HDR {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match self.world.get_hit_record(&ray, 0.00000001, INFINITY) {
            Some(record) => {
                let rand_ray = Ray::new(record.point, Vec3::rand_hemisphere_dir(record.normal));
                return self.shader_path_tracing(&rand_ray, depth - 1) * 0.2;
            }
            None => {
                return Color::new(1.0, 1.0, 1.0);
            }
        }
    }
}
