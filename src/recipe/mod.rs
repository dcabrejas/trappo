pub mod steps;

use self::steps::{Step, Context};
use super::display::*;
use std::process::exit;

pub struct Recipe {
    pub name : String,
    pub steps: Vec<Box<Step>>
}

pub struct RecipeExecutor;

impl RecipeExecutor {

    pub fn execute(recipe: &Recipe, context: &Context) -> () {

        render_success(&format!("🚀  Deploying to {} using '{}' recipe...", context.config.host, recipe.name));

        for step in recipe.steps.iter() {
            render_success(&format!("➜  Executing step {}...", step.get_name()));
            match step.execute(context) {
                Err(msg) => {
                    render_error(&format!("💣 Failed because of an IO error {}", msg));
                    exit(1);
                },
                Ok(_) => render_success(&format!("🗸  Step {} executed successfully", step.get_name()))
            }
        }
    }
}
