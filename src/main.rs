use clap::Parser;

mod study_cycle;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
