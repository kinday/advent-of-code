use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct InstructionParseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    AddX(i32),
    NoOp,
}

#[derive(Debug)]
struct ProgramParseError;

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>,
    cycle: (usize, usize, Option<Instruction>), // (index, skip, next_instruction)
    x: i32,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" ");

        match parts {
            Some(("addx", x)) => {
                let x = x.parse::<i32>().map_err(|_| InstructionParseError)?;

                Ok(Instruction::AddX(x))
            }
            None => match s {
                "noop" => Ok(Instruction::NoOp),
                _ => Err(InstructionParseError),
            },
            _ => Err(InstructionParseError),
        }
    }
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        let mut instructions = instructions.clone();

        let cycle = match instructions.pop() {
            Some(Instruction::AddX(x)) => (0, 3, Some(Instruction::AddX(x))),
            x => (0, 2, x),
        };

        Program {
            cycle,
            instructions,
            x: 1,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        self.x = match instruction {
            Instruction::AddX(x) => self.x + x,
            Instruction::NoOp => self.x,
        }
    }
}

impl FromStr for Program {
    type Err = ProgramParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .lines()
            .rev()
            .map(|line| line.parse::<Instruction>())
            .flatten()
            .collect();

        Ok(Program::new(instructions))
    }
}

impl Iterator for Program {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let (cycle, skip, next_instruction) = self.cycle.clone();

        if skip > 0 {
            self.cycle = (cycle + 1, skip - 1, next_instruction);
            return Some(self.x);
        }

        if let Some(next_instruction) = next_instruction {
            self.apply_instruction(next_instruction);
        }

        let instruction = self.instructions.pop();

        self.cycle = match instruction {
            None => (cycle, 0, None),
            Some(Instruction::AddX(x)) => (cycle + 1, 1, Some(Instruction::AddX(x))),
            x => (cycle + 1, 0, x),
        };

        match (instruction, next_instruction) {
            (Some(_), _) | (_, Some(_)) => Some(self.x),
            (None, None) => None,
        }
    }
}

fn solve_first(input: &String) -> String {
    let program = input.parse::<Program>().unwrap();
    program
        .zip(0..)
        .skip(20)
        .step_by(40)
        .map(|x| x.0 * x.1)
        .sum::<i32>()
        .to_string()
}

fn solve_second(input: &String) -> String {
    let program = input.parse::<Program>().unwrap();
    let pixels = program
        .skip(1)
        .zip((0..40).cycle())
        .map(|x| {
            let (sprite_position, index) = x;
            let pixel = if sprite_position.abs_diff(index) < 2 {
                "#".to_string()
            } else {
                ".".to_string()
            };
            if index >= 39 {
                pixel + "\n"
            } else {
                pixel
            }
        })
        .collect::<String>();

    pixels.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse_no_op() {
        let actual = "noop".parse::<Instruction>().unwrap();
        assert_eq!(actual, Instruction::NoOp);
    }

    #[test]
    fn test_instruction_parse_add_x_positive() {
        let actual = "addx 3".parse::<Instruction>().unwrap();
        assert_eq!(actual, Instruction::AddX(3));
    }

    #[test]
    fn test_instruction_parse_add_x_negative() {
        let actual = "addx -5".parse::<Instruction>().unwrap();
        assert_eq!(actual, Instruction::AddX(-5));
    }

    #[test]
    fn test_program_result_1() {
        let actual: Vec<i32> = "noop\naddx 3\naddx -5"
            .parse::<Program>()
            .unwrap()
            .collect();
        assert_eq!(actual, vec![1, 1, 1, 1, 4, 4, -1]);
    }

    #[test]
    fn test_program_result_2() {
        let actual: Vec<i32> = "addx 3\nnoop\naddx -5\nnoop"
            .parse::<Program>()
            .unwrap()
            .collect();
        assert_eq!(actual, vec![1, 1, 1, 4, 4, 4, -1, -1]);
    }

    #[test]
    fn test_solution_1() {
        let input = String::from("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
        let result = solve_first(&input);

        assert_eq!(result, "13140");
    }

    #[test]
    fn test_solution_2() {
        let input = String::from("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
        let result = solve_second(&input);

        // TODO: Fix unnecessary bonus pixel
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n.");
    }
}
