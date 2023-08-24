use rand::{thread_rng, Rng};
use std::f64::INFINITY;

use crate::bsdf::BSDF;
use crate::{
    hit::Hittable,
    light::*,
    maths::{Vec3, HDR},
    ray::Ray,
    renderer::Renderer,
};

pub enum ShaderType {
    PathTracing,
    LeadTest,
}

impl Renderer {
    pub fn shader_path_tracing(&self, ray: &Ray, depth: i32) -> HDR {
        let mut total_emmision = HDR::new(0.0, 0.0, 0.0);
        if depth < self.ctx.max_depth as i32 {
            for light in &self.light_group.lights {
                if let Light::SunLight(sunlight) = light {
                    let check_ray = Ray::new(ray.origin, sunlight.direction * (-1.0));
                    if let None = self.world.get_hit_record(&check_ray, 0.0001, INFINITY) {
                        total_emmision = total_emmision + sunlight.color * sunlight.intensity;
                    }
                }
            }
        }
        if depth <= 0 {
            return total_emmision / self.probability_rr;
        }
        match self.world.get_hit_record(&ray, 0.0001, INFINITY) {
            Some(record) => {
                let mut rng = thread_rng();
                let rand = rng.gen::<f64>();
                if rand > self.probability_rr {
                    return total_emmision / self.probability_rr;
                }
                let rand_dir = Vec3::rand_hemisphere_dir(record.normal);
                let rand_ray = Ray::new(record.point, rand_dir);
                let wi = rand_dir;
                let wo = ray.direction * (-1.0);
                let mut light_contrib = HDR::new(0.0, 0.0, 0.0);
                for light in &self.light_group.lights {
                    if let Light::SunLight(sunlight) = light {
                        let check_ray = Ray::new(ray.origin, sunlight.direction * (-1.0));
                        if let None = self.world.get_hit_record(&check_ray, 0.0001, INFINITY) {
                            light_contrib = light_contrib
                                + (sunlight.color * sunlight.intensity).mix(
                                    BSDF::cook_torrance_brdf(
                                        record.obj.get_roughness(),
                                        record.obj.get_reflectivity(),
                                        record.obj.get_roughness(),
                                        wo,
                                        wi,
                                        record.normal,
                                        record.obj.get_color(),
                                    ),
                                ) / self.probability_rr;
                        }
                    }
                }
                return light_contrib
                    + BSDF::cook_torrance_brdf(
                        record.obj.get_roughness(),
                        record.obj.get_reflectivity(),
                        record.obj.get_roughness(),
                        wo,
                        wi,
                        record.normal,
                        record.obj.get_color(),
                    )
                    .mix(self.shader_path_tracing(&rand_ray, depth - 1))
                        / self.probability_rr;
            }
            None => {
                for light in &self.light_group.lights {
                    if let Light::HDRILight(hdrilight) = light {
                        if let None = self.world.get_hit_record(ray, 0.0001, INFINITY) {
                            total_emmision =
                                total_emmision + hdrilight.get_hdr_value(ray.direction);
                        }
                    }
                }
                return total_emmision / self.probability_rr;
            }
        }
    }
}
