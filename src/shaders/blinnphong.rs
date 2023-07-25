use crate::{hit, maths::Color};

use super::Shader;

pub struct BlinnPhong {}

impl Shader for BlinnPhong {
    fn get_color_from_record(&self, record: hit::HitRecord) -> Color {
        todo!()
    }
}
