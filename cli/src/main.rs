use clap::Parser;
use miette::Result;
use std::path::PathBuf;
use core::estimate;

#[derive(Parser)]
struct Cli {
    /// Path to the Q# file to estimate
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Reading path from: {:?}", args.path);

    match estimate(&args.path) {
        Ok(estimate) => println!("Estimation result: {}", estimate),
        Err(error) => eprintln!("Error: {}", error),
    }

    Ok(())
}