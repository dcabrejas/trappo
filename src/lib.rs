extern crate toml_edit;
extern crate ansi_term;

pub mod recipe;
pub mod steps;
pub mod config;
mod display;
mod cmd;

use std::env;
use config::stages::parse_config_file;
use config::steps::{parse_steps_config_file, StepConfig};
use std::process::exit;
use steps::Context;
use std::error::Error;

pub fn init_stage_context() -> Context {

    let host_config =  match parse_config_file(".deploy-rs/config.toml", &get_environment()) {
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

pub fn init_steps_from_config() -> Vec<StepConfig> {
    parse_steps_config_file(".deploy-rs/steps.toml", &get_environment()).unwrap_or_else(|e| {
        eprintln!("steps config error: {}", e);
        exit(1);
    })
}

fn get_environment() -> String {
    env::args().nth(1).unwrap_or_else(|| {
        eprintln!("error: You need to pass the environment as the first argument, eg. cargo run development");
        exit(1);
    })
}
