use crate::{maths::Vec3, ray::Ray};

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
    fn get_hit(&self, ray: &Ray) -> Option<HitRecord>;
}
