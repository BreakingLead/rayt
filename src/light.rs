use crate::maths::{Point3, Vec3, Color};

pub struct LightGroup {
    lights: Vec<Light>,
}

impl LightGroup {
    pub fn new() -> Self {
        Self { lights: vec![] }
    }

    pub fn add(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn clear(&mut self) {
        self.lights.clear();
    }
}

pub struct Light {
    pub origin: Point3,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Point3, intensity: f64, color: Color) -> Self {
        Self { origin, intensity, color }
    }
}
