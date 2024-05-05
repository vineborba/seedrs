mod args;

use args::Args;
use clap::Parser;
use degit::options::Options;

fn main() {
    let cli = Args::parse();

    if let Err(err) = degit::run(Options::from(cli)) {
        eprintln!("Exited with error! Err: {err}")
    };
}
