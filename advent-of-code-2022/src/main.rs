use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

const CACHEDIR: &'static str = ".cache";

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

fn write_input_to_file(file_path: std::path::PathBuf, input: &String) -> Result<(), io::Error> {
    fs::create_dir_all(CACHEDIR)?;

    let file = fs::File::create(file_path)?;
    let mut file = io::LineWriter::new(file);

    file.write_all(input.as_bytes())?;
    file.flush()?;

    Ok(())
}

fn read_input_from_remote(day: String) -> Result<Vec<String>, ureq::Error> {
    let file_path = std::path::PathBuf::from(format!("{}/day_{}", CACHEDIR, day));

    #[cfg(debug_assertions)]
    eprintln!("Using {} as cache", file_path.to_string_lossy());

    if let Ok(cached_input) = read_input_from_file(file_path.clone()) {
        #[cfg(debug_assertions)]
        eprintln!("Found day {} in cache", day);
        return Ok(cached_input);
    };

    let token = env::var("AOC_TOKEN").expect("no $AOC_TOKEN defined");
    let path = format!("https://adventofcode.com/2022/day/{}/input", day);
    let cookie = format!("session={}", token);
    let body = ureq::get(&path)
        .set("Cookie", &cookie)
        .call()?
        .into_string()?;

    if let Err(_error) = write_input_to_file(file_path, &body) {
        #[cfg(debug_assertions)]
        eprintln!("Could not save day {} to cache", day);
        #[cfg(debug_assertions)]
        eprintln!("{}", _error);
    };

    let mut result = Vec::new();
    for line in body.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

fn solve(day: i8) -> (String, String) {
    let input: io::Result<Vec<String>> = match read_input_from_remote(day.to_string()).ok() {
        Some(input) => Ok(input),
        None => panic!("could not fetch"),
    };

    let raw_data = input.unwrap();
    return match day {
        1 => day_1::solve(raw_data),
        2 => day_2::solve(raw_data),
        3 => day_3::solve(raw_data),
        4 => day_4::solve(raw_data),
        5 => day_5::solve(raw_data.join("\n")),
        6 => day_6::solve(raw_data.join("\n")),
        7 => day_7::solve(raw_data.join("\n")),
        8 => day_8::solve(raw_data.join("\n")),
        _ => panic! {"bad day"},
    };
}

fn main() {
    let main_start = Instant::now();
    for day in 1..=8 {
        let day_start = Instant::now();
        let (answer_1, answer_2) = solve(day);
        let elapsed = day_start.elapsed();
        println!("Day {}, done in {:?}", day, elapsed);
        println!("\tPart 1: {}", answer_1);
        println!("\tPart 2: {}", answer_2);
        println!("")
    }
    println!("All done in {:?}", main_start.elapsed());
}
