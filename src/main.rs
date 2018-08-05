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
    let environment = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("error: You need to pass the environment as the first argument, eg. cargo run development");
        exit(1);
    });

    let host_config =  match parse_config_file("Deployer.toml", &environment) {
        Ok(context) => context,
        Err(config_error) => {
            eprintln!("config error: {}", config_error);
            exit(1);
        }
    };

    //println!("host config : {:#?}", host_config);
    //exit(0);

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
