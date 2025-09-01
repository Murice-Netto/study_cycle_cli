use clap::Parser;

use crate::commands::Commands;

mod commands;
mod handlers;
mod model;
mod study_cycle;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Study(args) => handlers::study_subject(args.name.clone()),
    }
}
