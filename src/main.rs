use clap::Parser;
use rcal::main_lib;

fn main() {
    let cli = rcal::cli::Cli::parse();
    let mut config = rcal::config::Config::build(&cli);

    main_lib::exec(&mut config);
}
