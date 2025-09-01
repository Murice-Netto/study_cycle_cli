use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Study(SutdyArgs),
    View(ViewArgs),
    Reset,
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
