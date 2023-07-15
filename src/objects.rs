use crate::hit::{Front, HitRecord, Hittable};
use crate::maths::Point3;
use crate::ray::Ray;

pub struct Sphere {
    radius: f64,
    center: Point3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { radius, center }
    }
}

impl Hittable for Sphere {
    fn get_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            None
        } else {
            let point = ray.at((-half_b - delta.sqrt()) / a);

            let outward_normal = (point - self.center).normalize();

            Some(HitRecord {
                point,
                normal: outward_normal,
                front_face: {
                    if ray.direction * outward_normal < 0.0 {
                        Front::Inward
                    } else {
                        Front::Outward
                    }
                },
            })
        }
    }
}
