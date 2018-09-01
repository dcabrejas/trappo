use steps::{Step, Context, core::*, git::*, composer::*};
use display::*;
use std::cell::RefCell;

pub struct Recipe {
    pub name : String,
    pub steps: Vec<Box<Step>>
}

impl Recipe {

    pub fn build() -> RecipeBuilder {
        RecipeBuilder {
            recipe: Some(RefCell::new(Recipe::default()))
        }
    }

    fn get_step_index(&self, step_name: &str) -> Option<usize> {
        let mut node_index: Option<usize> = None;

        let mut index: usize = 0;
        for step in &self.steps {
            if step.get_name() == step_name {
                node_index = Some(index);
                break;
            }
            index += 1;
        }

        node_index
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
    recipe: Option<RefCell<Recipe>>
}

impl RecipeBuilder {
    ///Set recipe name
    pub fn name(&mut self, name: &str) -> &mut Self {
        if let Some(ref mut recipe) = self.recipe {
            recipe.borrow_mut().name = name.into()
        }

        self
    }

    ///Set core steps
    pub fn with_core_steps(&mut self) -> &mut Self {
        if let Some(ref mut recipe) = self.recipe {
            let mut recipe_ref = recipe.borrow_mut();
            recipe_ref.steps.push(Box::new(InitStep));
            recipe_ref.steps.push(Box::new(GitClone));
            recipe_ref.steps.push(Box::new(ComposerInstall));
            recipe_ref.steps.push(Box::new(LinkFiles));
            recipe_ref.steps.push(Box::new(LinkDirs));
            recipe_ref.steps.push(Box::new(SymlinkCurrent));
            recipe_ref.steps.push(Box::new(CleanUpReleases));
        }

        self
    }

    ///Add step after another
    ///
    ///panics if step is not found in the inner vector
    pub fn with_step_after<T: 'static +  Step>(&mut self, subject_name: &str, extra_step: T) -> &mut Self {
        if let Some(ref mut recipe) = self.recipe {
            let step_index = recipe.borrow().get_step_index(subject_name)
                .expect(&format!("Step '{}' doesn't exist in the recipe", subject_name));

            recipe.borrow_mut().steps.insert(step_index + 1, Box::new(extra_step));
        }

        self
    }

    ///Add step before another
    ///
    ///panics if step is not found in the inner vector
    pub fn with_step_before<T: 'static +  Step>(&mut self, subject_name: &str, extra_step: T) -> &mut Self {
        if let Some(ref mut recipe) = self.recipe {
            let step_index = recipe.borrow().get_step_index(subject_name)
                .expect(&format!("Step '{}' doesn't exist in the recipe", subject_name));

            recipe.borrow_mut().steps.insert(step_index, Box::new(extra_step));
        }

        self
    }

    ///Consumes the recipe and returns it, leaving None as the value of the internal recipe property
    ///
    ///panics if recipe is attempted to be build multiple times.
    pub fn finish(&mut self) -> Recipe {
        let recipe = self.recipe.take().expect("Cannot reuse recipe builder");
        recipe.into_inner()
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
                    //exit(1);
                },
                Ok(_) => render_success(&format!("🗸  Step {} executed successfully", step.get_name()))
            }
        }
    }
}
