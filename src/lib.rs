extern crate toml_edit;
extern crate ansi_term;

pub mod recipe;

mod config;
mod display;
mod cmd;

use std::env;
use config::parse_config_file;
use std::process::exit;
use recipe::steps::Context;
use std::error::Error;

pub fn init_stage_context() -> Context {

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

    let context = match Context::from_host_config(host_config) {
        Ok(context) => context,
        Err(message) => {
            eprintln!("error: {:?}", message.description());
            exit(1);
        }
    };

    context
}
