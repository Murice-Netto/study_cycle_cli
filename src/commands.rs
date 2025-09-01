use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Study(SutdyArgs),
}

#[derive(Args)]
pub struct SutdyArgs {
    #[arg(short, long)]
    pub name: String,
}
