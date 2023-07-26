use std::ops::{Add, Div, Mul, Sub};
use core::f64::consts::PI;

use image::Rgb;
use rand::{thread_rng, Rng};

pub type Color = Vec3;
pub type HDR = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.clone() / self.length()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn gamma_correction(&self, gamma: f64) -> Color {
        Self::new(
            self.x.powf(1.0/gamma).clamp(0.0, 1.0), 
            self.y.powf(1.0/gamma).clamp(0.0, 1.0), 
            self.z.powf(1.0/gamma).clamp(0.0, 1.0),
        )
    }

    pub fn rand_hemisphere_dir(norm: Vec3) -> Vec3 {
        let mut rng = thread_rng();
        let theta = rng.gen::<f64>() * PI / 2.0;
        let phi = rng.gen::<f64>() * PI * 2.0;
        let rand_dir = Vec3::new(theta.sin()*phi.cos(), theta.sin()*phi.sin(), theta.cos());

        let normal = norm.normalize();
        let mut x = Vec3::new(1.0, 0.0, 0.0);
        let mut y = Vec3::new(0.0, 1.0, 0.0);
        let mut z = normal;
        if normal == Vec3::new(0.0, 0.0, 1.0) {
            x = Vec3::new(1.0, 0.0, 0.0);
        }
        else if normal == Vec3::new(0.0, 0.0, -1.0) {
            x = Vec3::new(-1.0, 0.0, 0.0);
        }
        else {
            x = Vec3::new(0.0, 0.0, 1.0).cross(&normal).normalize();
        }
        y = z.cross(&x).normalize();

        let rotated_dir = Vec3::new(
            x.x*rand_dir.x + y.x*rand_dir.y + z.x*rand_dir.z, 
            x.y*rand_dir.x + y.y*rand_dir.y + z.y*rand_dir.z, 
            x.z*rand_dir.x + y.z*rand_dir.y + z.z*rand_dir.z,
        ).normalize();

        rotated_dir
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(value: [f64; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div for Vec3 {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        self.x / rhs.x + self.y / rhs.y + self.z / rhs.z
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Into<Rgb<u8>> for Vec3 {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (self.x.clamp(0.0, 0.9999) * 256.0) as u8,
            (self.y.clamp(0.0, 0.9999) * 256.0) as u8,
            (self.z.clamp(0.0, 0.9999) * 256.0) as u8,
        ])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn normalize_test() {}
}
