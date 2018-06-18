extern crate toml_edit;
extern crate ansi_term;

mod config;
mod recipe;
mod display;

use std::env;
use config::*;
use recipe::{Recipe, RecipeExecutor};
use recipe::steps::Context;
use recipe::steps::core::get_steps;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let host_config =  parse_config_file().unwrap();
    let context = Context::from_host_config(host_config);

    let recipe = Recipe {
        name: "My first recipe".to_string(),
        steps: get_steps()
    };

    RecipeExecutor::execute(&recipe, &context);
}
