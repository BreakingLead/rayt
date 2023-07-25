
use crate::{hit, maths::Color};

use super::Shader;

pub struct WhittedStyleRayTracing {}

impl Shader for WhittedStyleRayTracing {
    fn get_color_from_record(&self, record: hit::HitRecord) -> Color {
        todo!()
    }
}
