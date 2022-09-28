#[allow(unused_imports)]
use clap::{ArgEnum, Parser, Subcommand};

fn main() {
    let cli = rcal::Cli::parse();

    rcal::exec(&cli);
}
