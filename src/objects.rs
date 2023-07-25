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
        Self { origin, edge_x, edge_y }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn edge_x(&self) -> Vec3 {
        self.edge_x
    }

    pub fn edge_y(&self) -> Vec3 {
        self.edge_y
    }
}

impl Hittable for Plane {
    fn get_hit_record(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //use the plane equation ax + by + cz + d = 0, where (a, b, c) is the normal vector of the plane.
        let mut norm = self.edge_x.cross(&self.edge_y).normalize();
        
        //solve for d
        let d = norm * self.origin * (-1.0);

        //calculate k in line parametric equation P_vec = O_vec + k * dir_vec
        let k = - (norm*ray.origin + d) / (norm*ray.direction);

        //identify whether k is in the acceptable range
        if k < t_min || k > t_max {
            return None;
        }

        //get intersection point
        let intersect_point = ray.origin + ray.direction * k;

        //calculate 4 vertices of the plane
        let vertice_1: Point3 = self.origin;
        let vertice_2: Point3 = self.origin + self.edge_x;
        let vertice_3: Point3 = self.origin + self.edge_x + self.edge_y;
        let vertice_4: Point3 = self.origin + self.edge_y;
        
        //calculate edge vector
        let edge_1: Vec3 = vertice_2 - vertice_1;
        let edge_2: Vec3 = vertice_3 - vertice_2;
        let edge_3: Vec3 = vertice_4 - vertice_3;
        let edge_4: Vec3 = vertice_1 - vertice_4;
        
        //calculate interesetion point vector relative to the four vertices
        let point_vec_1: Point3 = intersect_point - vertice_1;
        let point_vec_2: Point3 = intersect_point - vertice_2;
        let point_vec_3: Point3 = intersect_point - vertice_3;
        let point_vec_4: Point3 = intersect_point - vertice_4;

        //calculate the cross products of the edge vectors and the point vectors
        let result_1 = edge_1.cross(&point_vec_1).normalize();
        let result_2 = edge_2.cross(&point_vec_2).normalize();
        let result_3 = edge_3.cross(&point_vec_3).normalize();
        let result_4 = edge_4.cross(&point_vec_4).normalize();

        //collect results into an array
        let results = [result_1, result_2, result_3, result_4];
        
        //identify whether the four cross products are likely equal to the normal vector
        for result in results {
            if (result - norm).length() > 0.001 {
                return None;
            }
        }

        //flip the normal vector if necessary
        if ray.direction * norm > 0.0 {
            norm = norm * (-1.0);
        }

        return Some(HitRecord { 
            point: intersect_point, 
            normal: norm, 
            front_face: Front::Outward, 
            t: k,
        });
    }
}
