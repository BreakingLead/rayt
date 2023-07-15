use crate::maths::Color;
use crate::maths::Point3;
use crate::maths::Vec3;
use crate::objects::Hittable;
use crate::objects::Sphere;
use crate::IMAGE_HEIGHT;

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

    pub fn result(&self, obj: &impl Hittable) -> Color {
        match obj.hit_point(self) {
            Some(record) => {
                let normal = record.normal;
                let point = record.point;

                Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
            None => {
                let y_ratio = (self.direction.normalize().y + 1.0) / 2.0;

                Color::new(1.0, 1.0, 1.0) * (1.0 - y_ratio) + Color::new(0.5, 0.5, 0.7) * y_ratio
            }
        }
    }
}
