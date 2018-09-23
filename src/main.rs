extern crate deploy_rs;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use deploy_rs::recipe::{Recipe, deployer::*};
use docopt::Docopt;

const USAGE: &'static str = "
    deploy-rs

    Usage:
    deploy-rs deploy <stage>
    deploy-rs rollback <stage>
    deploy-rs (-h | --help)
    deploy-rs --version

    Options:
    -h --help     Show this screen.
    --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_stage: String,
    cmd_deploy: bool,
    cmd_rollback: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    let opt = if (args.cmd_deploy) { deploy_rs::Operation::Deploy } else { deploy_rs::Operation::Rollback };

    let stage_context = deploy_rs::init_stage_context(".deploy-rs/config.toml", &args.arg_stage, opt);
    let config_steps  = deploy_rs::init_steps_from_config(".deploy-rs/steps.toml", &args.arg_stage);

    let recipe = Recipe::build()
        .name("Main Recipe")
        .with_core_steps()
        .with_config_steps(config_steps)
        .with_core_rollback_steps()
        .finish();

    if args.cmd_deploy {
        deploy(&recipe, &stage_context);
    } else if args.cmd_rollback {
        rollback(&recipe, &stage_context);
    }
}
