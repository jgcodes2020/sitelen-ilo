use clap::Parser;

mod cli;
mod parse;

fn main() {
    let args = cli::Cli::parse();
}
