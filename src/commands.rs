use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "s")]
    Study(SutdyArgs),

    #[command(alias = "v")]
    View(ViewArgs),

    #[command(alias = "r")]
    Reset,
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
