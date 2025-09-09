use clap::{Args, Subcommand};

// Commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "s")]
    Study(SutdyArgs),

    #[command(alias = "v")]
    View(ViewArgs),

    #[command(alias = "r")]
    Reset,
}

// Args
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
