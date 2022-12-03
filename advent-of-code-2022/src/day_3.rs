use std::collections::HashSet;

type Compartment = String;
struct Rucksack(Compartment, Compartment);

type Group = Vec<Compartment>;

trait Intersectable {
    fn intersection(&self) -> char;
}

fn parse_line(line: &String) -> Rucksack {
    let length = line.len();
    let compartments = line.split_at(length / 2);
    let compartment_a = compartments.0.to_string();
    let compartment_b = compartments.1.to_string();
    return Rucksack(compartment_a, compartment_b);
}

fn parse_input(lines: Vec<String>) -> Vec<Rucksack> {
    return lines.iter().map(parse_line).collect();
}

fn find_common_item(rucksack: &Rucksack) -> char {
    for a in rucksack.0.chars() {
        for b in rucksack.1.chars() {
            if a == b {
                return b;
            }
        }
    }

    panic!("huh?")
}

fn to_priority(item: char) -> u32 {
    let code = item as u32;

    if item.is_ascii_lowercase() {
        return code - 97 + 1;
    }

    if item.is_ascii_uppercase() {
        return code - 65 + 27;
    }

    return 0;
}

fn solve_first(rucksacks: &Vec<Rucksack>) -> String {
    let items: u32 = rucksacks
        .iter()
        .map(find_common_item)
        .map(to_priority)
        .sum();
    return items.to_string();
}

fn parse_input_second(lines: Vec<String>) -> Vec<Group> {
    return lines
        .chunks(3)
        .map(|x| x.iter().map(|x| x.clone()).collect())
        .collect();
}

fn solve_second(groups: &Vec<Group>) -> String {
    let items: u32 = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                .reduce(|x, y| x.intersection(&y).map(|x| x.to_owned()).collect())
                .unwrap()
                .iter()
                .last()
                .unwrap()
                .clone()
        })
        .map(to_priority)
        .sum();
    return items.to_string();
}

pub fn solve(input: Vec<String>) -> (String, String) {
    // TODO: Get rid of double parsing
    let data = parse_input(input.clone());
    let data_second = parse_input_second(input);
    return (solve_first(&data), solve_second(&data_second));
}
