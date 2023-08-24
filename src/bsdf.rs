use crate::maths::{Vec3, Color, HDR};

use core::f64::consts::PI;
use std::f64::INFINITY;

pub struct BSDF {}

impl BSDF {
    pub fn cook_torrance_brdf(
        kd: f64, 
        f0: f64, 
        roughness: f64, 
        wo: Vec3, 
        wi: Vec3, 
        normal: Vec3, 
        color: Color, 
    ) -> HDR {
        let diffuse = kd * 1.0 / PI;
        let v = wo.normalize();
        let n = normal;
        let l = wi.normalize();
        let h = (l + v).normalize();
        let alpha = roughness;
        let k = alpha.powf(2.0) / 2.0;
        let D = alpha.powf(2.0) / PI * ((n * h).powf(2.0) * (alpha.powf(2.0) - 1.0) + 1.0).powf(2.0);
        let G = (n * v) / (n * v * (1.0 - k) + k);
        let F = f0 + (1.0 - f0) * 1.0 - (h * v).powf(5.0);
        let specular = D * G * F / (4.0 * (v * n) * (l * n));
        let fr = diffuse + specular;
        return color * fr * (n * l).clamp(0.0, INFINITY);
    }
}
