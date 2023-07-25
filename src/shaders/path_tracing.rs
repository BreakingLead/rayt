use crate::{hit, maths::Color};

use super::Shader;

pub struct PathTracing {}

impl Shader for PathTracing {
    fn get_color_from_record(&self, record: hit::HitRecord) -> Color {
        todo!()
    }
}
