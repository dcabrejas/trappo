extern crate deploy_rs;

use deploy_rs::recipe::{Recipe, RecipeExecutor};
use deploy_rs::recipe::steps::{Step, core};

fn main() {

    let stage_context = deploy_rs::init_stage_context();

    let extra_step = core::LinkDirs::new("core:link:directories");
    let extra_step_before = core::CleanUpReleases::new("core:cleanup:directories");

    let recipe = Recipe::build()
        .name("Main Recipe")
        .with_core_steps()
        .with_step_after("core:link:files", extra_step)
        .with_step_before("core:link:files", extra_step_before)
        .finish();

    RecipeExecutor::execute(&recipe, &stage_context);
}
