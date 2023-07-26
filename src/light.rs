use crate::maths::{Point3, Vec3, Color, HDR};
use crate::ray::Ray;
use crate::hit::{HitRecord, Front, Hittable};

use std::rc::Rc;

pub trait Light {}

pub struct LightGroup {
    pub lights: Vec<Rc<dyn Light>>,
}

impl LightGroup {
    pub fn new() -> Self {
        Self { lights: vec![] }
    }

    pub fn add(&mut self, light: Rc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn clear(&mut self) {
        self.lights.clear();
    }
}

pub struct HDRILight {
    pub intensity: f64,
}

impl HDRILight {
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }

    pub fn get_hdr_value(dir: Vec3) -> HDR {
        HDR::new(1.0, 1.0, 1.0)
    }
}

impl Light for HDRILight {}

pub struct SunLight {
    pub direction: Vec3,
    pub intensity: f64,
    pub color: Color,
}

impl SunLight {
    pub fn new(direction: Vec3, intensity: f64, color: Color) -> Self {
        Self { direction, intensity, color }
    }
}

impl Light for SunLight {}

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

impl Light for PointLight {}

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
        Self { origin, size, direction, focal_length, intensity, color }
    }
}

impl Light for SpotLight {}

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

impl Light for AreaLight {}

impl Hittable for AreaLight {
    fn get_hit_record(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // note: Figure <X> means that X is a vector.

        // == PROOF ==
        // point normal form equation of a plane:
        // [KNOWN] origin[O] = (x0, y0, z0) (this is the origin of the plane);
        //         normal[N] = ( A,  B,  C) ;
        //         point [P] = ( x,  y,  z) which is on the plane;
        //
        // [SOLVE] the equation
        //         obviously,
        //             N*(OP) = 0
        //             A(x-x0) + B(y-y0) + C(z-z0) = 0
        //         simplify it:
        //             Ax + By + Cz - (A x0 + B y0 + C z0) = 0
        //         which is equal to the normal form of a plane:
        //             Ax + By + Cz + D = 0

        // use the plane equation ax + by + cz + d = 0, where (A, B, C) is the normal vector of the plane
        let mut normal = self.edge_x.cross(&self.edge_y).normalize();

        // solve for d
        let d = normal * self.origin * (-1.0);

        // calculate k in line parametric equation <P> = <O> + t * <D>

        // == PROOF ==
        // [KNOWN]        P = O + tD  ; (O:  ray.origin)
        //         <MP> * N = 0       ; (M: self.origin)
        //                D = - (N*M) ; (N: self.normal)
        // [SOLVE] t
        //
        //         we can found:
        //             MP = P-M = O + tD - M
        //         so
        //             (O + tD - M) * N = 0
        //                tD*N + O*N - M*N = 0
        //                            t = (M*N - O*N) / D*N
        //                            t =  (-D - O*N) / D*N
        let t = (-d - ray.origin * normal) / (ray.direction * normal);

        // identify whether k is in the acceptable range
        if t < t_min || t > t_max {
            return None;
        }

        // get intersection point
        let intersection = ray.at(t);

        // calculate 4 vertices of the plane
        let vertices = self.vertices();

        // calculate edge vector
        let edges = [
            vertices[1] - vertices[0],
            vertices[2] - vertices[1],
            vertices[3] - vertices[2],
            vertices[0] - vertices[3],
        ];

        // calculate interesetion point vector relative to the four vertices
        let point_vecs = vertices.map(|v| intersection - v);

        let results: Vec<Vec3> = edges
            .iter()
            .enumerate()
            .map(|(i, v)| v.cross(&point_vecs[i]).normalize())
            .collect();

        // identify whether the four cross results are likely equal to the normal vector
        for result in results {
            if (result - normal).length() > 0.001 {
                return None;
            }
        }

        // flip the normal vector if necessary
        if ray.direction * normal > 0.0 {
            normal = normal * (-1.0);
        }

        return Some(HitRecord {
            obj: self,
            point: intersection,
            normal,
            front_face: Front::Outward,
            t,
        });
    }
}
