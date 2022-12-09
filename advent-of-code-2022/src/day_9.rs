use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct MoveParseError;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Move {
    N,
    NW,
    NE,
    S,
    SW,
    SE,
    W,
    E,
}

#[derive(Debug)]
struct MoveSeqParseError;

#[derive(Debug, PartialEq, Eq)]
struct MoveSeq {
    value: Vec<Move>,
}

#[derive(Debug)]
struct MovesParseError;

#[derive(Debug, PartialEq, Eq)]
struct Moves {
    value: Vec<Move>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position(i32, i32);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rope {
    head: Position,
    tail: Vec<Position>,
}

impl Position {
    fn new() -> Self {
        Position(0, 0)
    }

    fn apply_move(&self, m: &Move) -> Self {
        let next_x = match m {
            Move::W | Move::NW | Move::SW => self.0 - 1,
            Move::E | Move::NE | Move::SE => self.0 + 1,
            Move::N | Move::S => self.0,
        };
        let next_y = match m {
            Move::N | Move::NW | Move::NE => self.1 + 1,
            Move::S | Move::SW | Move::SE => self.1 - 1,
            Move::W | Move::E => self.1,
        };

        Position(next_x, next_y)
    }

    fn apply_pull(&self, m: &Move, origin: &Position) -> Self {
        let distance = self.as_abs_distance(origin);

        if distance < 2 {
            return self.to_owned();
        }

        let delta_x = (origin.0 - self.0).clamp(-1, 1);
        let delta_y = (origin.1 - self.1).clamp(-1, 1);

        self.apply_move(match (m, delta_x, delta_y) {
            (_, -1, 1) => &Move::NW,
            (_, 1, 1) => &Move::NE,
            (_, -1, -1) => &Move::SW,
            (_, 1, -1) => &Move::SE,
            (_, 0, 1) => &Move::N,
            (_, 0, -1) => &Move::S,
            (_, -1, 0) => &Move::W,
            (_, 1, 0) => &Move::E,
            _ => panic!("Argh"),
        })
    }

    fn as_abs_distance(&self, origin: &Position) -> i32 {
        let distance_x = origin.0.abs_diff(self.0);
        let distance_y = origin.1.abs_diff(self.1);

        if distance_x > distance_y {
            distance_x.try_into().unwrap()
        } else {
            distance_y.try_into().unwrap()
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            head: Position::new(),
            tail: std::iter::repeat(Position::new()).take(length).collect(),
        }
    }

    fn apply_move(&self, m: &Move) -> Self {
        let head = self.head.apply_move(m);
        let mut tail = Vec::new();

        for position in self.tail.iter() {
            let prev_knot = if tail.len() == 0 {
                head
            } else {
                tail.last().cloned().unwrap()
            };
            let next_position = position.apply_pull(m, &prev_knot);
            tail.push(next_position);
        }

        Rope { head, tail }
    }

    fn as_tail_position(&self) -> Option<Position> {
        self.tail.iter().last().cloned()
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row_count: i32 = 47;
        let column_count: i32 = 166;
        let mut rows: Vec<String> = Vec::new();
        for row in 0..row_count {
            let mut columns = Vec::new();
            for column in 0..column_count {
                if row == 0 {
                    columns.push(((column - column_count / 2) % 10).abs().to_string());
                    continue;
                }

                if column == 0 {
                    columns.push(((row - row_count / 2) % 10).abs().to_string());
                    continue;
                }

                let point = Position(column - column_count / 2, row - row_count / 2);
                let symbol = if self.head == point {
                    String::from("H")
                } else if let Some(tail) = self.tail.iter().enumerate().find(|p| p.1 == &point) {
                    (tail.0 + 1).to_string()
                    // String::from("X")
                } else {
                    String::from("•")
                };
                columns.push(symbol);
            }
            rows.push(columns.join(""));
        }
        rows.reverse();
        let result: String = rows.join("\n");
        write!(f, "{}", result)
    }
}

impl FromStr for Move {
    type Err = MoveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Move::N),
            "D" => Ok(Move::S),
            "L" => Ok(Move::W),
            "R" => Ok(Move::E),
            _ => Err(MoveParseError),
        }
    }
}

impl FromStr for MoveSeq {
    type Err = MoveSeqParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (move_type, count) = s.split_once(" ").ok_or(MoveSeqParseError)?;

        let move_type = move_type.parse::<Move>().map_err(|_| MoveSeqParseError)?;
        let count = count.parse::<usize>().map_err(|_| MoveSeqParseError)?;

        let value = std::iter::repeat(move_type).take(count).collect();

        Ok(MoveSeq { value })
    }
}

impl FromStr for Moves {
    type Err = MovesParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .lines()
            .flat_map(|line| line.parse::<MoveSeq>())
            .flat_map(|move_seq| move_seq.value)
            .collect();

        Ok(Moves { value })
    }
}

fn solve_first(input: &String) -> String {
    let moves = input.parse::<Moves>().unwrap();
    let mut rope = Rope::new(1);
    let mut tail_moves = HashSet::new();
    for m in moves.value {
        let next_rope = rope.apply_move(&m);
        tail_moves.insert(next_rope.as_tail_position().unwrap());
        rope = next_rope;
    }
    tail_moves.len().to_string()
}

fn solve_second(input: &String) -> String {
    let moves = input.parse::<Moves>().unwrap();
    let mut rope = Rope::new(9);
    let mut tail_moves = HashSet::new();
    for m in moves.value {
        let next_rope = rope.apply_move(&m);
        tail_moves.insert(next_rope.as_tail_position().unwrap());
        rope = next_rope;
    }
    tail_moves.len().to_string()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"

    #[test]
    fn test_moves_from_str() {
        let moves = "U 1\nD 1\nL 1\nR 1".parse::<Moves>().unwrap();
        assert_eq!(
            moves,
            Moves {
                value: vec![Move::N, Move::S, Move::W, Move::E]
            }
        );
    }

    #[test]
    fn test_moves_from_str_repeating() {
        let moves = "U 2\nD 3\nL 1\nR 4".parse::<Moves>().unwrap();
        assert_eq!(
            moves,
            Moves {
                value: vec![
                    Move::N,
                    Move::N,
                    Move::S,
                    Move::S,
                    Move::S,
                    Move::W,
                    Move::E,
                    Move::E,
                    Move::E,
                    Move::E
                ]
            }
        );
    }

    #[test]
    fn test_position_apply_move() {
        let moves = Moves {
            value: vec![Move::N, Move::N, Move::W, Move::W],
        };
        let position = moves
            .value
            .iter()
            .fold(Position::new(), |p, m| p.apply_move(m));

        assert_eq!(position, Position(-2, 2));
    }

    #[test]
    fn test_position_as_distance() {
        let position_a = Position(3, -2);
        let position_b = Position(-1, -1);
        let distance = position_a.as_abs_distance(&position_b);

        assert_eq!(distance, 4);
    }

    #[test]
    fn test_rope_apply_move_1() {
        let moves = "U 4\nR 4".parse::<Moves>().unwrap();
        let rope = moves
            .value
            .iter()
            .fold(Rope::new(4), |r, m| r.apply_move(m));

        let expected = Rope {
            head: Position(4, 4),
            tail: vec![
                Position(3, 4),
                Position(2, 4),
                Position(2, 3),
                Position(2, 2),
            ],
        };

        assert_eq!(rope, expected);
    }

    #[test]
    fn test_rope_apply_move_2() {
        let moves = "R 5\nU 8".parse::<Moves>().unwrap();
        let rope = moves
            .value
            .iter()
            .fold(Rope::new(4), |r, m| r.apply_move(m));

        let expected = Rope {
            head: Position(5, 8),
            tail: vec![
                Position(5, 7),
                Position(5, 6),
                Position(5, 5),
                Position(5, 4),
            ],
        };

        assert_eq!(rope, expected);
    }

    #[test]
    fn test_rope_apply_move_3() {
        let moves = "R 5\nU 8\nL 5".parse::<Moves>().unwrap();
        let rope = moves
            .value
            .iter()
            .fold(Rope::new(4), |r, m| r.apply_move(m));

        let expected = Rope {
            head: Position(0, 8),
            tail: vec![
                Position(1, 8),
                Position(2, 8),
                Position(3, 7),
                Position(3, 6),
            ],
        };

        assert_eq!(rope, expected);
    }

    #[test]
    fn test_rope_apply_move_4() {
        let moves = "U 4\nR 5\nD 2".parse::<Moves>().unwrap();
        let rope = moves
            .value
            .iter()
            .fold(Rope::new(4), |r, m| r.apply_move(m));

        let expected = Rope {
            head: Position(5, 2),
            tail: vec![
                Position(5, 3),
                Position(4, 3),
                Position(3, 3),
                Position(2, 2),
            ],
        };

        assert_eq!(rope, expected);
    }

    #[test]
    fn test_solution_1() {
        let input = String::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
        let result = solve_first(&input);

        assert_eq!(result, "13");
    }

    #[test]
    fn test_solution_2_short() {
        let input = String::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
        let result = solve_second(&input);

        assert_eq!(result, "1");
    }

    #[test]
    fn test_solution_2_long() {
        let input = String::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20");
        let result = solve_second(&input);

        assert_eq!(result, "36");
    }
}
