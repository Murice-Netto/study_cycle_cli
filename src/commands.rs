use clap::{Args, Subcommand};

// Commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "s", about = "Studies a subject by it's name")]
    Study(StudyArgs),

    #[command(alias = "v", about = "Shows subjects that you can still study")]
    View(ViewArgs),

    #[command(
        alias = "r",
        about = "Resets the study cycle's subject's studied_hours to zero"
    )]
    Reset,
}

// Args
#[derive(Args, Debug)]
pub struct ViewArgs {
    #[arg(short, long, help = "Shows all subjects, even the completed ones")]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct StudyArgs {
    #[arg(help = "Name of your subject")]
    pub name: String,
}
