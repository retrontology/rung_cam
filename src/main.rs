mod camera;
mod config;

use crate::camera::Camera;
use crate::config::Config;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args: Cli = Cli::parse();
    let config: Config = Config::from_file(&args.config);

    println!("index is {}", &config.get_camera_index());
}
