use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::process::ExitCode;

fn run(args: Vec<String>) -> Result<()> {
    let file = &args[1];

    let contents = fs::read_to_string(file)
        .with_context(|| format!("failed to read input from {file}"))?;

    println!("Read {} bytes", contents.len());

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} input.txt", args[0]);
        return ExitCode::from(1);
    }

    if let Err(e) = run(args) {
        println!("error running: {e:?}");
        return ExitCode::from(1);
    };

    ExitCode::SUCCESS
}
