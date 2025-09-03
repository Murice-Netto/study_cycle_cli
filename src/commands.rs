use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "s")]
    Study(SutdyArgs),

    #[command(alias = "v")]
    View(ViewArgs),

    #[command(alias = "r")]
    Reset,

    #[command(name = "seed")]
    SeedDatabase(SeedDatabaseArgs),
}

#[derive(Args, Debug)]
pub struct ViewArgs {
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct SutdyArgs {
    #[arg()]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct SeedDatabaseArgs {
    #[arg()]
    pub path: String,
}
