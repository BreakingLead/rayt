use std::rc::Rc;

use camera::Camera;
use light::Light;

use crate::{hit::HittableList, objects::{Sphere, Plane}, renderer::{Renderer, Shader}};

mod camera;
mod hit;
mod light;
mod maths;
mod objects;
mod ray;
mod utils;
mod renderer;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;
const SAMPLES_PER_PIXEL: u32 = 30;

fn main() {
    //create camera
    let camera = Camera::new();

    //create light
    let light = Light::new([-8.0, 8.0, -6.0].into(), 1.0, [1.0, 1.0, 1.0].into());

    //create world with objects
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new([3.0, 1.0, -10.0].into(), 1.0)));
    world.add(Rc::new(Sphere::new([-3.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-6.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-4.5, 2.5, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([0.0, 2.0, -2.0].into(), 1.0)));
    world.add(Rc::new(Plane::new([-8.0, -2.0, -5.0].into(), 
                                    [16.0, -1.0, 0.0].into(), 
                                    [0.0, 8.0, -10.0].into())));

    //render
    let renderer = Renderer::new(world, light, camera, Shader::PathTracing);
    let img = renderer.render(true);

    //output image
    img.save("test.png").unwrap();
}
