use crate::maths::{Color, Point3};
use crate::ray::Ray;
use crate::Vec3;

pub enum Front {
    Inward,
    Outward,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: Front,
}

pub trait Hittable {
    fn hit_point(&self, ray: &Ray) -> Option<HitRecord>;
}

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
    fn hit_point(&self, ray: &Ray) -> Option<HitRecord> {
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
                front_face: { if ray.direction * outward_normal < 0.0 {Front::Inward} else {Front::Outward} },
            })
        }
    }
}
