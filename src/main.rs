extern crate toml_edit;
extern crate ansi_term;

mod config;
mod recipe;
mod display;
mod cmd;

use std::env;
use config::parse_config_file;
use recipe::{Recipe, RecipeExecutor};
use recipe::steps::*;
use std::process::exit;
use std::error::Error;

fn main() {
    let _args: Vec<String> = env::args().collect();

    let host_config =  match parse_config_file("Deployer.toml", "staging") {
        Ok(context) => context,
        Err(message) => {
            eprintln!("error: {:?}", message.description());
            exit(1);
        }
    };

    let context = match Context::from_host_config(host_config) {
        Ok(context) => context,
        Err(message) => {
            eprintln!("error: {:?}", message.description());
            exit(1);
        }
    };

    let recipe = Recipe {
        name: "My first recipe".to_string(),
        steps: get_steps()
    };

    RecipeExecutor::execute(&recipe, &context);
}
