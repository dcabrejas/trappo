extern crate deploy_rs;

use deploy_rs::recipe::{Recipe, RecipeExecutor};
use deploy_rs::steps::core;
use deploy_rs::config::steps::*;

fn main() {

    let stage_context = deploy_rs::init_stage_context();
    let config_steps  = deploy_rs::init_steps_from_config();

    let mut recipe_builder = Recipe::build();
    recipe_builder.name("Main Recipe").with_core_steps();

    for step_config in config_steps.into_iter() {
        match step_config.position {
            StepPosition::Before => {
                recipe_builder.with_step_before(&step_config.ref_step.to_owned(), core::RawCmdStep::from(step_config));
            },
            StepPosition::After => {
                recipe_builder.with_step_after(&step_config.ref_step.to_owned(), core::RawCmdStep::from(step_config));
            }
        };
    }

    let recipe = recipe_builder.finish();

    RecipeExecutor::execute(&recipe, &stage_context);
}
