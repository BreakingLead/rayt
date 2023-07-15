use crate::{
    maths::{Point3, Vec3},
    ray::Ray,
    IMAGE_HEIGHT, IMAGE_WIDTH,
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;
        let focal_length: f64 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, x_ratio: f64, y_ratio: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x_ratio + self.vertical * y_ratio
                - self.origin,
        )
    }
}
