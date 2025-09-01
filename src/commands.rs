use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Study(SutdyArgs),
    View(ViewArgs),
    Reset,
    SeedDatabase(SeedDatabaseArgs),
}

#[derive(Args)]
pub struct ViewArgs {
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args)]
pub struct SutdyArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(Args)]
pub struct SeedDatabaseArgs {
    #[arg(short, long)]
    pub path: String,
}
