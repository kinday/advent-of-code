fn parse_input(data: Vec<String>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut subresult = Vec::new();

    for datum in data {
        if datum == "" {
            result.push(subresult.clone());
            subresult.clear();
        } else {
            subresult.push(datum.parse::<i32>().unwrap())
        }
    }

    if subresult.len() > 0 {
        result.push(subresult);
    }

    return result;
}

fn to_sums(elves: Vec<Vec<i32>>) -> Vec<i32> {
    let mut elf_sums = Vec::new();

    for elf in elves {
        let mut sum = 0;
        for item in elf {
            sum += item
        }
        elf_sums.push(sum)
    }

    return elf_sums;
}

pub fn solve_first(input: Vec<String>) -> String {
    let elves = parse_input(input);

    let elf_sums = to_sums(elves);

    let max = elf_sums.into_iter().max();

    let result = match max {
        Some(x) => x,
        None => panic! {"who is Max?"},
    };

    return result.to_string();
}

pub fn solve_second(input: Vec<String>) -> String {
    let elves = parse_input(input);

    let mut elf_sums = to_sums(elves);

    elf_sums.sort();

    let top_three_sum: i32 = elf_sums.into_iter().rev().take(3).sum();

    return top_three_sum.to_string();
}

pub fn solve(part: String, input: Vec<String>) -> String {
    return match part.as_str() {
        "A" => solve_first(input),
        "B" => solve_second(input),
        _ => panic! {"not my part"},
    };
}
