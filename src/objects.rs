use crate::hit::{Front, HitRecord, Hittable};
use crate::maths::{Point3, Vec3};
use crate::ray::Ray;

pub struct Sphere {
    radius: f64,
    center: Point3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { radius, center }
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

pub struct Plane {
    origin: Point3,
    edge_x: Vec3,
    edge_y: Vec3,
}

impl Plane {
    pub fn new(origin: Point3, edge_x: Vec3, edge_y: Vec3) -> Self {
        Self {
            origin,
            edge_x,
            edge_y,
        }
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

impl Hittable for Plane {
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
            point: intersection,
            normal,
            front_face: Front::Outward,
            t,
        });
    }
}
