mod args;

use anyhow::Result;
// use args::Args;
// use clap::Parser;
// use seedrs::Options;
use std::process::exit;

fn main() -> Result<()> {
    // let args = Args::parse();

    if let Err(err) = seedrs::run(/* Options::from(args)*/ ) {
        eprintln!("An error ocurred: {err}");
        exit(1)
    }

    Ok(())
}
