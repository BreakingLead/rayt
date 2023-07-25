use crate::{hit::HitRecord, maths::Color, renderer::Renderer};

pub enum ShaderType {
    Phong,
    Etc,
}

impl Renderer {
    pub fn shader_phong(&self, record: HitRecord) -> Color {
        // TODO: shader
        Color::new(1.0, 1.0, 1.0)
    }
}
