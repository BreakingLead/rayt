use crate::{hit, maths::Color};

use super::Shader;

pub struct Phong {}

impl Phong {
    pub fn new() -> Self {
        Self {}
    }
}

impl Shader for Phong {
    fn get_color_from_record(&self, record: hit::HitRecord) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}
