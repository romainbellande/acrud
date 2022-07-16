// https://github.com/clap-rs/clap/blob/v3.2.8/examples/tutorial_builder/03_04_subcommands.rs
mod fixtures;

use clap::{command, Command};
use dotenv::dotenv;

pub async fn exec() {
    dotenv().ok();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("fixtures:load")
                .about("Load fixtures into database specified with DATABASE_URL environment variable"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("fixtures:load", _)) => {
            fixtures::load().await;
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
