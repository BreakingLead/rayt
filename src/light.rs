use crate::maths::{Color, Point3};

pub struct Light {
    pub origin: Point3,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Point3, intensity: f64, color: Color) -> Self {
        Self {
            origin,
            intensity,
            color,
        }
    }
}
