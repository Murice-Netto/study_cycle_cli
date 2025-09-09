use clap::Parser;

use crate::commands::Commands;
use crate::error::AppError;

mod commands;
mod error;
mod handlers;
mod model;
mod storage;
mod study_cycle;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    let storage: Box<dyn storage::Storage> =
        Box::new(storage::json_storage::JsonStorage::new("./database.json"));
    let handler: Box<dyn handlers::Handler> = Box::new(handlers::v1::V1Handler::new(storage));

    match &cli.command {
        Commands::Study(args) => handler.study_cycle_by_name(&args.name)?,
        Commands::View(args) => handler.show_cycle(args.all)?,
        Commands::Reset => handler.reset_cycle()?,
    };

    Ok(())
}
