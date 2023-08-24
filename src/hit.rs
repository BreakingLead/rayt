use std::rc::Rc;

use crate::{maths::{Vec3, Color}, ray::Ray};

pub enum Front {
    Inward,
    Outward,
}

pub struct HitRecord<'a> {
    pub obj: &'a dyn Hittable,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: Front,
    pub t: f64,
}

pub trait Hittable {
    fn get_hit_record(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn get_color(&self) -> Color;
    fn get_roughness(&self) -> f64;
    fn get_reflectivity(&self) -> f64;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn get_hit_record(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result = None;
        for object in &self.objects {
            match object.get_hit_record(ray, t_min, closest) {
                Some(rec) => {
                    closest = rec.t;
                    result = Some(rec);
                }
                None => continue,
            }
        }
        result
    }
    fn get_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn get_roughness(&self) -> f64 {
        0.0
    }
    fn get_reflectivity(&self) -> f64 {
        0.0
    }
}
