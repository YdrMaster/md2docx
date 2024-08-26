mod convert;
mod show;

#[macro_use]
extern crate clap;
use clap::Parser;

fn main() {
    use Commands::*;
    match Cli::parse().command {
        Show(args) => args.show(),
        Convert(args) => args.convert(),
    }
}

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Show(show::Args),
    Convert(convert::Args),
}
