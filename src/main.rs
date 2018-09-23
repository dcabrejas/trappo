extern crate trappo;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use trappo::recipe::{Recipe, deployer::*};
use docopt::Docopt;

const USAGE: &'static str = "
    trappo

    Usage:
    trappo deploy <stage>
    trappo rollback <stage>
    trappo (-h | --help)
    trappo --version

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
    let opt = if args.cmd_deploy { trappo::Operation::Deploy } else { trappo::Operation::Rollback };

    let stage_context = trappo::init_stage_context(".trappo/config.toml", &args.arg_stage, opt);
    let config_steps  = trappo::init_steps_from_config(".trappo/steps.toml", &args.arg_stage);

    let recipe = Recipe::build()
        .name("Main Recipe")
        .with_core_steps()
        .with_config_steps(config_steps)
        .with_core_rollback_steps()
        .finish();

    if args.cmd_deploy {
        deploy(&recipe, &stage_context).unwrap();
    } else if args.cmd_rollback {
        rollback(&recipe, &stage_context).unwrap()
    }
}
