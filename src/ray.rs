use std::f64::INFINITY;

use crate::hit::Hittable;
use crate::maths::{Color, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn get_pixel_color<T: Hittable>(&self, obj: &T) -> Color {
        match obj.get_hit_record(self, 0.0, INFINITY) {
            Some(record) => {
                let normal = record.normal;

                Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
            None => {
                let y_ratio = (self.direction.normalize().y + 1.0) / 2.0;

                Color::new(1.0, 1.0, 1.0) * (1.0 - y_ratio) + Color::new(0.5, 0.5, 0.7) * y_ratio
            }
        }
    }
}
