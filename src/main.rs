use clap::Parser;

mod model;
mod study_cycle;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
