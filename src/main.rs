use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::io::Error as IOError;
use std::process::exit;

use clap::Parser;

use space_sum::*;
use crate::error::SpaceSumError;

mod error;

#[derive(Parser, Debug)]
#[command(author, version, about = "Read size expressions like 200M or 2G and compute the sum.", long_about = None)]
struct Args {
    #[arg(help = "The file containing sizes to sum. If omitted, values are read from stdin.")]
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    let sum = match args.filename {
        None => sum_from_stdin(),
        Some(filename) => sum_file(&filename),
    };

    match sum {
        Ok(sum) => {
            let size = human_readable_size(sum);
            println!("{size}");
        }

        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    }
}

fn sum_file(filename: &str) -> Result<f64, SpaceSumError> {
    let fd = File::open(filename)?;
    let buffer = BufReader::new(fd);

    sum_from_lines(Box::new(buffer))
}

fn sum_from_stdin() -> Result<f64, SpaceSumError> {
    sum_from_lines(Box::new(stdin().lock()))
}

fn sum_from_lines(buffer: Box<dyn BufRead>) -> Result<f64, SpaceSumError> {
    buffer
        .lines()
        .map(|line| parse_size_line(line))
        .sum()
}

fn parse_size_line (line: Result<String, IOError>) -> Result<f64, SpaceSumError> {
    match line {
        Ok(expression) => match parse_size(&expression) {
            None => {
                let error = format!("Can't parse size expression: {expression}");
                Err(SpaceSumError::Parser(error))
            },
            Some(size) => Ok(size),
        },

        Err(error) => Err(SpaceSumError::IO(error)),
    }
}
