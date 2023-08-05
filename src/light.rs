use crate::maths::{Point3, Vec3, Color, HDR};
use crate::ray::Ray;
use crate::hit::{HitRecord, Front, Hittable};

pub enum Light {
    HDRILight(HDRILight),
    SunLight(SunLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
    AreaLight(AreaLight),
}

pub struct LightGroup {
    pub lights: Vec<Light>,
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

pub struct HDRILight {
    pub color: Color,
    pub intensity: f64,
}

impl HDRILight {
    pub fn new(color: Color, intensity: f64) -> Self {
        Self { color, intensity }
    }

    pub fn get_hdr_value(&self, dir: Vec3) -> HDR {
        let hdr = self.color * self.intensity;
        hdr
    }
}

pub struct SunLight {
    pub direction: Vec3,
    pub intensity: f64,
    pub color: Color,
}

impl SunLight {
    pub fn new(direction: Vec3, intensity: f64, color: Color) -> Self {
        let direction = direction.normalize();
        Self { direction, intensity, color }
    }
}

pub struct PointLight {
    pub origin: Point3,
    pub size: f64,
    pub intensity: f64,
    pub color: Color,
}

impl PointLight {
    pub fn new(origin: Point3, size: f64, intensity: f64, color: Color) -> Self {
        Self { origin, size, intensity, color }
    }
}

pub struct SpotLight {
    pub origin: Point3,
    pub size: f64,
    pub direction: Vec3,
    pub focal_length: f64,
    pub intensity: f64,
    pub color: Color,
}

impl SpotLight {
    pub fn new(origin: Point3, size: f64, direction: Vec3, focal_length: f64, intensity: f64, color: Color) -> Self {
        let direction = direction.normalize();
        Self { origin, size, direction, focal_length, intensity, color }
    }
}

pub struct AreaLight {
    pub origin: Point3,
    pub edge_x: Vec3,
    pub edge_y: Vec3,
    pub intensity: f64,
    pub color: Color,
}

impl AreaLight {
    pub fn new(origin: Point3, edge_x: Vec3, edge_y: Vec3, intensity: f64, color: Color) -> Self {
        Self { origin, edge_x, edge_y, intensity, color }
    }

    pub fn vertices(&self) -> [Vec3; 4] {
        [
            self.origin,
            self.origin + self.edge_x,
            self.origin + self.edge_x + self.edge_y,
            self.origin + self.edge_y,
        ]
    }
}
