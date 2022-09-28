use clap::Parser;

fn main() {
    let cli = rcal::Cli::parse();

    rcal::exec(&cli);
}
