pub mod steps;

use recipe::steps::{Step, Context, core};
use display::*;
use std::process::exit;

pub struct Recipe {
    pub name : String,
    pub steps: Vec<Box<Step>>
}

impl Recipe {

    pub fn build() -> RecipeBuilder {
        RecipeBuilder {
            recipe: Some(Recipe::default())
        }
    }
}

impl Default for Recipe {
    fn default() -> Recipe {
        Recipe {
            name: "Anonymous Recipe".into(),
            steps: Vec::new()
        }
    }
}

pub struct RecipeBuilder {
    recipe: Option<Recipe>
}

impl RecipeBuilder {
    ///Set recipe name
    pub fn name(&mut self, name: &str) -> &mut Self {
        if let Some(recipe) = self.recipe.as_mut() {
            recipe.name = name.into()
        }

        self
    }

    ///Set core steps
    pub fn with_core_steps(&mut self) -> &mut Self {
        if let Some(recipe) = self.recipe.as_mut() {
            recipe.steps.push(Box::new(core::SetUpStep::new("core:setup")));
            recipe.steps.push(Box::new(core::LinkFiles::new("core:link:files")));
            //recipe.steps.push(Box::new(core::LinkDirs::new("core:link:directories")));
            //recipe.steps.push(Box::new(core::SymlinkCurrent::new("core:link:current")));
            //recipe.steps.push(Box::new(core::CleanUpReleases::new("core:cleanup:releases")));
        }

        self
    }

    ///Add step after another
    ///
    ///panics if step is not found in the inner vector (todo)
    pub fn with_step_after<T: 'static +  Step>(&mut self, node: &str, extra_step: T) -> &mut Self {
        if let Some(recipe) = self.recipe.as_mut() {

            //todo look at abstracting this.
            let mut node_index: Option<usize> = None;
            let mut index: usize = 0;
            for step in &recipe.steps {

                if step.get_name() == node {
                    node_index = Some(index);
                    break;
                }

                index += 1;
            }

            match node_index {
                Some(index) => recipe.steps.insert(index + 1, Box::new(extra_step)),
                None => {
                    let error = format!("Step '{}' doesn't exist in the recipe", node);
                    panic!(error.to_owned());
                }
            }
        }

        self
    }

    ///Add step before another
    ///
    ///panics if step is not found in the inner vector (todo)
    pub fn with_step_before<T: 'static +  Step>(&mut self, node: &str, extra_step: T) -> &mut Self {
        if let Some(recipe) = self.recipe.as_mut() {

            //todo look at abstracting this.
            let mut node_index: Option<usize> = None;
            let mut index: usize = 0;
            for step in &recipe.steps {

                if step.get_name() == node {
                    node_index = Some(index);
                    break;
                }

                index += 1;
            }

            match node_index {
                Some(index) => recipe.steps.insert(index, Box::new(extra_step)),
                None => {
                    let error = format!("Step '{}' doesn't exist in the recipe", node);
                    panic!(error.to_owned());
                }
            }
        }

        self
    }

    //Why not something like `let node_index: Option<usize> = None; for (index, step) in &recipe.steps.enumerate() { if step.get_name() == node { let node_index = index; break; } } recipe.steps.insert(node_index, extra_step);`?

    pub fn finish(&mut self) -> Recipe {
        let recipe = self.recipe.take().expect("cannot request recipe builder");
        recipe
    }
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
