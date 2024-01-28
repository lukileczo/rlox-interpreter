#![allow(unused)]

use clap::Parser;

use rlox_interpreter::Lox;

#[derive(Parser, Debug)]
struct Opts {
    /// script filename
    #[arg(short, long)]
    filename: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    let mut lox = Lox::new();
    match opts.filename {
        Some(filename) => lox.run_file(filename),
        None => lox.run_prompt(),
    }
}
