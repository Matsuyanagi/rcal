use clap::Parser;

fn main() {
    let mut cli = rcal::Cli::parse();

    rcal::exec(&mut cli);
}
