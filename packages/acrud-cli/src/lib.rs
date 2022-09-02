// https://github.com/clap-rs/clap/blob/v3.2.8/examples/tutorial_builder/03_04_subcommands.rs
use acrud::{db::Database, fixtures};
use api;
use clap::{command, Command};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};

pub async fn exec() {
    dotenv().ok();

    let matches =
        command!()
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("fixtures:load").about(
                "Load fixtures into database specified with DATABASE_URL environment variable",
            ))
            .subcommand(Command::new("migration:migrate").about("Execute database migrations"))
            .get_matches();

    match matches.subcommand() {
        Some(("fixtures:load", _)) => {
            fixtures::load(api::fixtures::service()).await;
        }
        Some(("migration:migrate", _)) => {
            let database = Database::new().await;

            let conn = database.connection;

            Migrator::up(&conn, None)
                .await
                .expect("could not migrate database");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
