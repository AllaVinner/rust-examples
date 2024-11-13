use clap::Parser;
mod basic;
mod cli;

fn main() {
    let cli = cli::CLI::parse();
    cli.run();
}
