use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} input.txt", args[0]);
        return ExitCode::from(1);
    }

    let file = &args[1];

    let contents = fs::read_to_string(file).expect("error reading file");

    println!("Read {} bytes", contents.len());

    ExitCode::SUCCESS
}
