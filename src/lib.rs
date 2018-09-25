extern crate ansi_term;
extern crate toml_edit;

mod cmd;
pub mod config;
mod display;
pub mod recipe;
pub mod steps;

use config::stages::parse_config_file;
use config::steps::{parse_steps_config_file, StepConfig};
use std::error::Error;
use std::process::exit;
use steps::Context;

#[derive(Clone, Copy)]
pub enum Operation {
    Deploy,
    Rollback,
}

pub fn init_stage_context(config_file: &str, stage: &str, opt: Operation) -> Context {
    let host_config = match parse_config_file(config_file, stage) {
        Ok(context) => context,
        Err(config_error) => {
            eprintln!("config error: {}", config_error);
            exit(1);
        }
    };

    match Context::from_host_config(host_config, opt) {
        Ok(context) => context,
        Err(message) => {
            eprintln!("error: {:?}", message.description());
            exit(1);
        }
    }
}

pub fn init_steps_from_config(config_file: &str, stage: &str) -> Vec<StepConfig> {
    parse_steps_config_file(config_file, stage).unwrap_or_else(|e| {
        eprintln!("steps config error: {}", e);
        exit(1);
    })
}
