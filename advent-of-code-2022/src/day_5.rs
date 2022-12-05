struct Stack {
    value: Vec<char>,
}

struct Stacks {
    value: Vec<Stack>,
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

struct Moves {
    value: Vec<Move>,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let cleaned = s
            .replace("move ", "")
            .replace(" from ", " ")
            .replace(" to ", " ");
        let mut data = cleaned.split(" ");
        let count = data.next().unwrap().parse::<usize>().unwrap();
        let from = data.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = data.next().unwrap().parse::<usize>().unwrap() - 1;
        Move { count, from, to }
    }
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        Moves {
            value: s.split('\n').map(|line| Move::from(line)).collect(),
        }
    }
}

impl Stack {
    fn push(&mut self, c: char) {
        self.value.push(c);
    }
    fn pop(&mut self) -> char {
        self.value.pop().expect("no crate to unload")
    }
}

impl Stacks {
    fn apply_move(&mut self, m: &Move, at_once: bool) -> &mut Self {
        let value: &mut Vec<Stack> = self.value.as_mut();
        let from_stack = value.get_mut(m.from).expect("no stack to read from");
        let mut crates = Vec::new();
        for _ in 0..m.count {
            crates.push(from_stack.pop());
        }
        if at_once {
            crates.reverse()
        }
        let to_stack = value.get_mut(m.to).expect("no stack to write to");
        crates.iter().for_each(|c| to_stack.push(c.to_owned()));

        self
    }
}

impl From<&str> for Stacks {
    fn from(s: &str) -> Self {
        let mut value: Vec<Stack> = Vec::new();
        let (stack_count, stack_data) = s
            .split("\n")
            .map(|line| line.to_string())
            .map(|line| -> Vec<char> {
                line.chars()
                    // Fisrt character is always either a space or a bracket
                    .skip(1)
                    // Every fourth character is actual data
                    .step_by(4)
                    // Drop last row of numbers
                    .filter(|c| !c.is_numeric())
                    .collect()
            })
            .fold((0, Vec::new()), |acc, line| {
                if line.len() > 0 {
                    (line.len(), [line, acc.1].concat())
                } else {
                    acc
                }
            });

        for _ in 1..=stack_count {
            value.push(Stack {
                value: stack_data
                    .iter()
                    .skip(value.len())
                    .step_by(stack_count)
                    .map(|c| c.clone())
                    .filter(|c| c.is_alphabetic())
                    .collect(),
            })
        }

        Stacks { value }
    }
}

fn parse_input(data: &String) -> (Stacks, Moves) {
    let (raw_stacks, raw_moves) = data.split_once("\n\n").expect("no input separator");

    (Stacks::from(raw_stacks), Moves::from(raw_moves))
}

fn solve_first(input: &String) -> String {
    let (mut stacks, moves) = parse_input(input);
    moves
        .value
        .iter()
        .fold(&mut stacks, |s: &mut Stacks, m| {
            let result = s.apply_move(m, false);
            result
        })
        .value
        .iter()
        .map(|stack| stack.value.iter().last().unwrap())
        .collect()
}

fn solve_second(input: &String) -> String {
    let (mut stacks, moves) = parse_input(input);
    moves
        .value
        .iter()
        .fold(&mut stacks, |s: &mut Stacks, m| {
            let result = s.apply_move(m, true);
            result
        })
        .value
        .iter()
        .map(|stack| stack.value.iter().last().unwrap())
        .collect()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}
