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

use serde::Deserialize;

use crate::objects::plane::SerializationPlane;
use crate::{
    camera::*, const_vars::ConstContext, hit::HittableList, light::*, objects::*, renderer::*,
    shaders::ShaderType,
};

use crate::objects::sphere::SerializationSphere;
#[derive(Deserialize)]
pub struct Config {
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    #[serde(rename = "Sphere")]
    spheres: Vec<SerializationSphere>,
    #[serde(rename = "Plane")]
    planes: Vec<SerializationPlane>,
}

pub fn init() -> ConstContext {
    let config = fs::read_to_string(Path::new("config.toml")).unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    ConstContext {
        samples_per_pixel: config.samples,
        max_depth: config.max_depth,
        output: true,
        config,
    }
}

pub fn draw(ctx: ConstContext) {
    //create camera
    let camera = Camera::new(ctx.config.width, ctx.config.height);

    //create light group
    let mut light_group = LightGroup::new();
    light_group.add(Light::HDRILight(HDRILight::new(
        [1.0, 1.0, 1.0].into(),
        1.0,
    )));
    /*light_group.add(Light::SunLight(SunLight::new(
        [1.0, -0.6, -1.0].into(),
        1.0,
        [1.0, 1.0, 1.0].into(),
    )));
    light_group.add(Light::PointLight(PointLight::new(
        [-8.0, 8.0, -6.0].into(),
        0.5,
        1.0,
        [1.0, 1.0, 1.0].into(),
    )));*/

    //create world with objects
    let mut world = HittableList::new();

    for i in &ctx.config.spheres {
        world.add(Rc::new(Sphere::from(i)));
    }

    for i in &ctx.config.planes {
        world.add(Rc::new(Plane::from(i)));
    }

    //render
    let renderer = Renderer::new(
        world, 
        light_group, 
        camera, 
        ctx, 
        ShaderType::PathTracing, 
        0.8, 
        2.2
    );
    let img = renderer.render();

    //output image
    img.save("test.png").unwrap();
}
