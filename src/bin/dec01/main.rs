use anyhow::{format_err, Context, Result};
use std::env;
use std::fs;
use std::process::ExitCode;

fn run(args: Vec<String>) -> Result<()> {
    let file = &args[1];

    let contents = fs::read_to_string(file)
        .with_context(|| format!("failed to read input from {file}"))?;

    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();

        let item1 = parts.next().ok_or_else(|| format_err!("line {line:?} missing first item"))?;
        let item2 = parts.next().ok_or_else(|| format_err!("line {line:?} missing second item"))?;

        let i1 = item1.parse::<i64>()
            .with_context(|| format!("failed to parse {item1:?}"))?;
        let i2 = item2.parse::<i64>()
            .with_context(|| format!("failed to parse {item1:?}"))?;

        list1.push(i1);
        list2.push(i2);
    }

    list1.sort();
    list2.sort();

    let dist: i64 = list1.iter().zip(list2.iter())
        .fold(0, |acc, (i1, i2)| {
            let dist = (i1 - i2).abs();
            acc + dist
        });

    println!("Distance: {dist}");

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
