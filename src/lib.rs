extern crate toml_edit;
extern crate ansi_term;

pub mod recipe;
pub mod steps;
pub mod config;
mod display;
mod cmd;

use config::stages::parse_config_file;
use config::steps::{parse_steps_config_file, StepConfig};
use std::process::exit;
use steps::Context;
use std::error::Error;

pub enum Operation {
    Deploy,
    Rollback
}

pub fn init_stage_context(config_file: &str, stage: &str, opt: Operation) -> Context {

    let host_config =  match parse_config_file(config_file, stage) {
        Ok(context) => context,
        Err(config_error) => {
            eprintln!("config error: {}", config_error);
            exit(1);
        }
    };

    let context = match Context::from_host_config(host_config, opt) {
        Ok(context) => context,
        Err(message) => {
            eprintln!("error: {:?}", message.description());
            exit(1);
        }
    };

    context
}

pub fn init_steps_from_config(config_file: &str, stage: &str) -> Vec<StepConfig> {
    parse_steps_config_file(config_file, stage).unwrap_or_else(|e| {
        eprintln!("steps config error: {}", e);
        exit(1);
    })
}
