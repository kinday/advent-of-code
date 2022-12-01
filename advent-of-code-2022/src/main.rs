use std::io;

mod day_1;

struct Cli {
    day: i8,
    part: String,
}

fn solve(day: i8, part: String, input: Vec<String>) -> String {
    return match day {
        1 => day_1::solve(part, input),
        _ => panic! {"bad day"},
    };
}

fn main() {
    let day = std::env::args().nth(1).expect("no day given");
    let part = std::env::args().nth(2).expect("no part given");

    let args = Cli {
        day: day.parse::<i8>().unwrap(),
        part,
    };

    let lines = io::stdin().lines();
    let mut input = Vec::new();
    for line in lines {
        input.push(line.unwrap());
    }

    println!("{}", solve(args.day, args.part, input))
}
