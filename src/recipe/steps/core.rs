use super::super::super::config::HostConfig;
use std::process::Command;
use super::{Step, Context};

pub struct SetUpStep { name: &'static str }

impl Step for SetUpStep {
    fn new(name: &'static str) -> SetUpStep {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), &str> {
        let server_command = format!(
            "mkdir -p {}",
            context.release_path
        );

        let _output = Command::new("ssh")
            .args(&[&context.config.host, server_command.as_str()])
            .output()
            .expect("Failed to execute set up step");

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//define another step
pub struct SayByeStep { name: &'static str }

impl Step for SayByeStep {

    fn new(name: &'static str) -> SayByeStep {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), &str> {
        println!("Bye bye World!");
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub fn get_steps() -> Vec<Box<Step>>{

    let hi_step = SetUpStep::new("core:setup");
    let bye_step = SayByeStep::new("bye");

    let mut steps: Vec<Box<Step>> = Vec::new();
    steps.push(Box::new(hi_step));
    steps.push(Box::new(bye_step));

    steps
}