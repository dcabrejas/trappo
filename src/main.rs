extern crate toml_edit;

mod config;
mod recipe;

use std::env;
use config::*;
use recipe::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host_config =  parse_config_file().unwrap();
    //todo pass host_config by ref and reuse it for every step
    let steps: Vec<recipe::SayHiStep> = recipe::get_steps(host_config);

    //execute the steps
    for step in steps.iter() {
        step.execute();
    }
}
