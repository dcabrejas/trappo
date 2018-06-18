use std::process::{Command, Stdio};
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

pub struct GitClone { name: &'static str }

impl Step for GitClone {

    fn new(name: &'static str) -> GitClone {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), &str> {
        let server_command = format!(
            "cd {} && git clone {} .",
            context.release_path.trim(),
            context.config.repo_url.trim()
        );

        let _output = Command::new("ssh")
            .args(&[&context.config.host, server_command.as_str()])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute git clone step");

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub struct ComposerInstall { name: &'static str }

impl Step for ComposerInstall {

    fn new(name: &'static str) -> ComposerInstall {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), &str> {
        let server_command = format!(
            "cd {} && composer install",
            context.release_path.trim()
        );

        let _output = Command::new("ssh")
            .args(&[&context.config.host, server_command.as_str()])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute composer install command step");

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub fn get_steps() -> Vec<Box<Step>>{

    let core_setup = SetUpStep::new("core:setup");
    let git_clone = GitClone::new("git:clone");
    let composer_install = ComposerInstall::new("composer:install");

    let mut steps: Vec<Box<Step>> = Vec::new();
    steps.push(Box::new(core_setup));
    steps.push(Box::new(git_clone));
    steps.push(Box::new(composer_install));

    steps
}