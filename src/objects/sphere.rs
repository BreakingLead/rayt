use crate::hit::{Front, HitRecord, Hittable};
use crate::maths::{Point3, Vec3, Color};
use crate::ray::Ray;

pub struct Sphere {
    radius: f64,
    center: Point3,
    color: Color,
    roughness: f64,
    reflectivity: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: Color, roughness: f64, reflectivity: f64) -> Self {
        Self {
            radius,
            center,
            color,
            roughness,
            reflectivity,
        }
    }
}

impl Hittable for Sphere {
    fn get_hit_record(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;

        if delta < 0.0 {
            return None;
        }

        let mut root = (-half_b - delta.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + delta.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;

        Some(HitRecord {
            point: ray.at(root),
            normal: outward_normal,
            front_face: {
                if ray.direction * outward_normal < 0.0 {
                    Front::Inward
                } else {
                    Front::Outward
                }
            },
            t: root,
        })
    }
}
