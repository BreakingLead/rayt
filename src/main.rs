use cli::draw;
use cli::init;

mod camera;
mod cli;
mod const_vars;
mod hit;
mod light;
mod maths;
mod objects;
mod ray;
mod renderer;
mod shaders;
mod bsdf;

fn main() {
    let ctx = init();
    draw(ctx);
}
