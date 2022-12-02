use std::env;
use std::fs;
use std::io::{self, prelude::*};

mod day_1;
mod day_2;

struct Cli {
    day: i8,
    filename: Option<std::path::PathBuf>,
    part: String,
    remote: bool,
}

fn solve(day: i8, part: String, input: Vec<String>) -> String {
    return match day {
        1 => day_1::solve(part, input),
        2 => day_2::solve(part, input),
        _ => panic! {"bad day"},
    };
}

fn read_input_from_file(file_path: std::path::PathBuf) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        if let Err(error) = line {
            return Err(error);
        }

        result.push(line.unwrap())
    }

    return Ok(result);
}

fn read_input_from_stdin() -> io::Result<Vec<String>> {
    let lines = io::stdin().lines();
    let mut result = Vec::new();
    for line in lines {
        if let Err(error) = line {
            return Err(error);
        }

        result.push(line.unwrap())
    }

    return Ok(result);
}

fn read_input_from_remote(day: String) -> Result<Vec<String>, ureq::Error> {
    let token = env::var("AOC_TOKEN").expect("no $AOC_TOKEN defined");
    let path = format!("https://adventofcode.com/2022/day/{}/input", day);
    let cookie = format!("session={}", token);
    let body = ureq::get(&path)
        .set("Cookie", &cookie)
        .call()?
        .into_string()?;

    let mut result = Vec::new();
    for line in body.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

fn parse_args() -> Cli {
    let day = env::args().nth(1).expect("no day given");
    let part = env::args().nth(2).expect("no part given");
    let filename = env::args().nth(3).unwrap_or(String::from(""));
    let remote = filename.eq("");

    return Cli {
        day: day.parse::<i8>().unwrap(),
        filename: match filename.as_str() {
            "" => None,
            "-" => None,
            _ => Some(std::path::PathBuf::from(filename)),
        },
        part,
        remote,
    };
}

fn main() {
    let args = parse_args();
    let input: io::Result<Vec<String>> = match args.filename {
        Some(file_path) => read_input_from_file(file_path),
        None => {
            if args.remote {
                match read_input_from_remote(args.day.to_string()).ok() {
                    Some(input) => Ok(input),
                    None => panic!("could not fetch"),
                }
            } else {
                read_input_from_stdin()
            }
        }
    };
    let raw_data = input.unwrap();
    println!("The answer is {}", solve(args.day, args.part, raw_data))
}
