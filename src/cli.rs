// use std::path::PathBuf;

// use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// pub struct Cli {
//     /// Sets a config file
//     #[arg(short, long, value_name = "FILE")]
//     config: Option<PathBuf>,

//     #[command(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Run tracer
//     Run {
//         #[arg(short, long, value_name = "SHADER")]
//         shader: Option<String>,
//     },
// }

// pub fn execute_args() {
//     let cli = Cli::parse();

//     if let Some(path) = cli.config.as_deref() {
//     }

// }

use std::{fs, path::Path, rc::Rc};

use crate::{
    camera::*, const_vars::ConstContext, hit::HittableList, light::*, objects::*, renderer::*,
};

pub fn init() -> ConstContext {
    let config = fs::read_to_string(Path::new("../config.toml")).unwrap();
    
    ConstContext {
        samples_per_pixel: 10,
        output: true,
    }
}

pub fn draw(ctx: ConstContext) {
    //create camera
    let camera = Camera::new(400, 300);

    //create light
    let light = Light::new([-8.0, 8.0, -6.0].into(), 1.0, [1.0, 1.0, 1.0].into());

    //create world with objects
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new([3.0, 1.0, -10.0].into(), 1.0)));
    world.add(Rc::new(Sphere::new([-3.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-6.0, 1.0, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([-4.5, -2.5, -10.0].into(), 3.0)));
    world.add(Rc::new(Sphere::new([0.0, 2.0, -2.0].into(), 1.0)));
    world.add(Rc::new(Plane::new(
        [-8.0, -2.0, -5.0].into(),
        [16.0, -1.0, 0.0].into(),
        [0.0, 8.0, -10.0].into(),
    )));

    //render
    let renderer = Renderer::new(world, light, camera, Shader::PathTracing, ctx);
    let img = renderer.render();

    //output image
    img.save("test.png").unwrap();
}
