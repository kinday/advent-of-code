use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::time::Instant;

mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

#[derive(Debug)]
struct SolutionError<T>(T);

impl std::error::Error for SolutionError<day_1::SolutionError> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Display for SolutionError<day_1::SolutionError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to solve day 1, part 1")
    }
}

impl From<day_1::SolutionError> for SolutionError<day_1::SolutionError> {
    fn from(error: day_1::SolutionError) -> Self {
        SolutionError(error)
    }
}

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

fn solve(day: i8) -> Result<(String, String), Box<dyn std::error::Error>> {
    let input: io::Result<Vec<String>> = match read_input_from_remote(day.to_string()).ok() {
        Some(input) => Ok(input),
        None => panic!("could not fetch"),
    };

    let raw_data = input.unwrap();
    return match day {
        1 => {
            let input = raw_data.join("\n");
            let part_1 = day_1::solve_first(&input)?;
            let part_2 = day_1::solve_second(&input)?;
            Ok((part_1, part_2))
        }
        2 => Ok(day_2::solve(raw_data)),
        3 => Ok(day_3::solve(raw_data)),
        4 => Ok(day_4::solve(raw_data)),
        5 => Ok(day_5::solve(raw_data.join("\n"))),
        6 => Ok(day_6::solve(raw_data.join("\n"))),
        7 => Ok(day_7::solve(raw_data.join("\n"))),
        8 => Ok(day_8::solve(raw_data.join("\n"))),
        9 => Ok(day_9::solve(raw_data.join("\n"))),
        10 => {
            let (answer_1, answer_2) = day_10::solve(raw_data.join("\n"));

            // Add some tabulation for better output
            let answer_2: String = answer_2
                .lines()
                .map(|line| ["\t\t", line, "\n"].concat())
                .collect();

            // Remove tabulation on first line
            let answer_2 = answer_2.trim_start().to_string();

            Ok((answer_1, answer_2))
        }
        11 => {
            let input = raw_data.join("\n");
            let part_1 = day_11::solve_first(&input)?;
            let part_2 = day_11::solve_second(&input)?;
            Ok((part_1, part_2))
        }
        _ => panic! {"bad day"},
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_start = Instant::now();
    for day in 1..=11 {
        let day_start = Instant::now();
        let result = solve(day);
        let elapsed = day_start.elapsed();
        println!("Day {}, done in {:?}", day, elapsed);
        match result {
            Ok((answer_1, answer_2)) => {
                println!("\tPart 1: {}", answer_1);
                println!("\tPart 2: {}", answer_2);
            }
            Err(error) => {
                eprintln!("\tError: {}", error);
            }
        };
        println!("");
    }
    println!("All done in {:?}", main_start.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_fixture_from_file(day: i32) -> String {
        let file_path = std::path::PathBuf::from(format!("{}/day_{}", "fixtures", day));
        read_input_from_file(file_path).unwrap().join("\n")
    }

    #[test]
    fn test_solve_day_1_part_1_ok() {
        let input = read_fixture_from_file(1);
        let result = day_1::solve_first(&input).unwrap();
        assert_eq!(result, String::from("24000"));
    }

    #[test]
    fn test_solve_day_1_part_1_err() {
        let input = String::from("BOOM");
        let result = day_1::solve_first(&input);
        assert_eq!(
            result,
            Err(day_1::SolutionError::TroupeParseError(
                day_1::TroupeParseError
            ))
        );
    }

    #[test]
    fn test_solve_day_1_part_2_ok() {
        let input = read_fixture_from_file(1);
        let result = day_1::solve_second(&input).unwrap();
        assert_eq!(result, String::from("45000"));
    }

    #[test]
    fn test_solve_day_1_part_2_err() {
        let input = String::from("BOOM");
        let result = day_1::solve_second(&input);
        assert_eq!(
            result,
            Err(day_1::SolutionError::TroupeParseError(
                day_1::TroupeParseError
            ))
        );
    }

    #[test]
    fn test_solve_day_11_part_1_ok() {
        let input = read_fixture_from_file(11);
        let result = day_11::solve_first(&input).unwrap();
        assert_eq!(result, String::from("10605"));
    }

    #[test]
    fn test_solve_day_11_part_2_ok() {
        let input = read_fixture_from_file(11);
        let result = day_11::solve_second(&input).unwrap();
        assert_eq!(result, String::from("2713310158"));
    }
}
