use crate::maths::{Point3, Vec3, Color};

pub struct Light {
    pub origin: Point3,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Point3, intensity: f64, color: Color) -> Self {
        let res = Self {
            origin,
            intensity,
            color,
        };

        res
    }
}