use crate::cli::Config;

pub struct ConstContext {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub output: bool,
    pub config: Config,
}
