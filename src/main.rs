use clap::Parser;

mod cli;
mod ast;
mod parse;

fn main() {
    let args = cli::Cli::parse();
    args.preinit_all();
}
