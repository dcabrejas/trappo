use super::Recipe;
use display::*;
use steps::{error::StepError, Context, Step};

pub fn deploy(recipe: &Recipe, context: &Context) -> Result<(), ()> {
    render_success(&format!(
        "🚀  Deploying to {} using '{}' recipe...",
        context.config.host, recipe.name
    ));

    execute_steps(&recipe.steps, &context).or_else(|err_msg| {
        render_error(&format!("💣 Critical error : {}", err_msg));
        rollback(&recipe, &context)
    })
}

pub fn rollback(recipe: &Recipe, context: &Context) -> Result<(), ()> {
    render_success(&format!(
        "🤦‍  Rolling back on {} using '{}' recipe...",
        context.config.host, recipe.name
    ));

    execute_steps(&recipe.rollback_steps, &context).or_else(|err_msg| {
        render_error(&format!(
            "💣 Could not roll back because of critical error : {}",
            err_msg
        ));
        Err(())
    })
}

fn execute_steps(steps: &[Box<dyn Step>], context: &Context) -> Result<(), String> {
    for step in steps.iter() {
        render_success(&format!("➜  Executing step {}...", step.get_name()));

        match step.execute(context) {
            Ok(_) => render_success(&format!(
                "🗸  Step {} executed successfully",
                step.get_name()
            )),
            Err(step_error) => match step_error {
                StepError::Critical(msg) => return Err(msg),
                StepError::NonCritical(_) => {
                    render_error(&"Non-critical error, continuing".to_string())
                }
            },
        };
    }

    Ok(())
}
