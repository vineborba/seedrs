mod args;

use anyhow::Result;
use args::Args;
use bootstrapper::Options;
use clap::Parser;
use std::process::exit;

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(err) = bootstrapper::run(Options::from(args)) {
        eprintln!("An error ocurred: {err}");
        exit(1)
    }

    Ok(())
}
