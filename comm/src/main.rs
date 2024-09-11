#![forbid(unsafe_code)]

use std::{io, process};
use std::path::PathBuf;
use comm::FileContent;

fn main() -> Result<(), io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let paths = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let first = FileContent::try_from(paths.0.as_path()).unwrap_or_else( |err| {
        eprintln!("Unable to read content from the first file: {err}");
        process::exit(1);
    });
    let second = FileContent::try_from(paths.1.as_path()).unwrap_or_else( |err| {
        eprintln!("Unable to read content from the second file: {err}");
        process::exit(1);
    });
    for line in first.common_lines(&second) {
        eprintln!("{line}");
    }
    Ok(())
}

fn parse_args(args: &Vec<String>) -> Result<(PathBuf, PathBuf), &'static str> {
    if args.len() != 3 {
        let mut args_str = String::new();
        for arg in args {
            args_str.push_str(arg);
            args_str.push('\n');
        }
        return Err("expected exactly 3 arguments");
    }
    Ok((PathBuf::from(&args[1]), PathBuf::from(&args[2])))
}
