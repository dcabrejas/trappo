use super::config::HostConfig;

pub trait Step {
    fn new(name: &'static str, config: HostConfig) -> Self;
    fn execute(&self) -> Result<(), &str>;
}

pub struct SayHiStep { name: &'static str, config: HostConfig }

impl Step for SayHiStep {
    fn new(name: &'static str, config: HostConfig) -> SayHiStep {
        Self { name, config }
    }

    fn execute (&self) -> Result<(), &str> {
        println!("Hello World!");
        Ok(())
    }
}

pub fn get_steps(config: HostConfig) -> Vec<SayHiStep>{

    let hi_step = SayHiStep::new("hi", config);

    let mut steps = Vec::new();
    steps.push(hi_step);
    steps
}