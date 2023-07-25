use crate::{
    maths::{Point3, Vec3},
    ray::Ray,
    IMAGE_HEIGHT, IMAGE_WIDTH,
};

#[derive(Debug)]
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

        let res = Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };

        res
    }

    pub fn get_ray(&self, width: u32, height: u32, x: f64, y: f64) -> Ray {
        let x_ratio = x as f64 / (width - 1) as f64;
        let y_ratio = y as f64 / (height - 1) as f64;
        Ray::new(
            self.origin,
            (self.lower_left_corner + self.horizontal * x_ratio + self.vertical * y_ratio
                - self.origin)
                .normalize(),
        )
    }
}
