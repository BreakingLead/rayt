use crate::maths::Color;

use crate::hit::HitRecord;

pub trait Shader {
    fn get_color_from_record(&self, record: HitRecord) -> Color;
}

mod blinnphong;
mod path_tracing;
mod phong;
mod whitted_style_ray_tracing;

pub use blinnphong::BlinnPhong;
pub use path_tracing::PathTracing;
pub use phong::Phong;
pub use whitted_style_ray_tracing::WhittedStyleRayTracing;
