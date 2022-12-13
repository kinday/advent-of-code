use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
};

type Elevation = i32;

enum Marker {
    Start,
    End,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    elevation: Elevation,
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize, elevation: Elevation) -> Self {
        Position { x, y, elevation }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    moves: HashMap<Position, Elevation>,
    position: Position,
}

#[derive(Debug)]
struct ElevationMap {
    _from: usize,
    _moves: HashMap<usize, Vec<usize>>,
    _positions: Vec<Position>,
    _to: usize,
}

impl ElevationMap {
    fn new() -> Self {
        ElevationMap {
            _from: 0,
            _moves: HashMap::new(),
            _positions: Vec::new(),
            _to: 0,
        }
    }

    fn add_move(&mut self, from_index: usize, to_index: usize) {
        let from = self.get_position(from_index);
        let to = self.get_position(to_index);
        let is_valid = match (from, to) {
            (Some(from), Some(to)) => {
                let elevation_diff = to.elevation - from.elevation;
                elevation_diff <= 1
            }
            _ => panic!("That’s weird"),
        };

        if is_valid {
            self._moves
                .entry(from_index)
                .and_modify(|positions| positions.push(to_index))
                .or_insert(vec![to_index]);
        }
    }

    fn add_position(&mut self, position: Position, marker: Option<Marker>) {
        let next_index = self._positions.len();

        match marker {
            Some(Marker::Start) => {
                self._from = next_index;
            }
            Some(Marker::End) => {
                self._to = next_index;
            }
            None => {}
        };

        self._positions.push(position);
    }

    fn get_moves(&self, key: &usize) -> Option<&Vec<usize>> {
        self._moves.get(key)
    }

    fn get_position(&self, index: usize) -> Option<&Position> {
        self._positions.get(index)
    }

    fn get_start(&self) -> usize {
        self._from
    }

    fn get_end(&self) -> usize {
        self._to
    }

    fn find_shortest_path(&self, from: &usize, to: &usize) -> Option<Vec<&Position>> {
        let mut distance = HashMap::from([(from, 0)]);
        let mut predecessors: HashMap<&usize, &usize> = HashMap::new();
        let mut visited = HashSet::from([from]);
        let mut queue = VecDeque::from([from]);

        while let Some(node) = queue.pop_front() {
            if let Some(adjacents) = self.get_moves(node) {
                for adjacent in adjacents {
                    if !visited.contains(adjacent) {
                        visited.insert(adjacent);
                        distance
                            .entry(adjacent)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                        predecessors.insert(adjacent, node);
                        queue.push_back(adjacent);

                        if adjacent == to {
                            break;
                        }
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current: &usize = to;

        while let Some(predecessor) = predecessors.get(current).cloned() {
            path.push(predecessor.clone());
            current = predecessor;
        }

        path.reverse();

        if path.len() == 0 {
            None
        } else {
            Some(
                path.iter()
                    .map(|index| self.get_position(index.clone()))
                    .flatten()
                    .collect(),
            )
        }
    }

    fn positions(&self) -> std::slice::Iter<Position> {
        self._positions.iter()
    }
}

impl FromStr for ElevationMap {
    type Err = SolutionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elevation_map = ElevationMap::new();

        // Collect all positions first, we can derive moves from them
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let (marker, ch) = match ch {
                    'S' => (Some(Marker::Start), 'a'),
                    'E' => (Some(Marker::End), 'z'),
                    ch => (None, ch),
                };
                let position = Position::new(x, y, ch as i32);
                elevation_map.add_position(position, marker);
            }
        }

        let rows = s.lines().count();
        let columns = elevation_map.positions().count() / rows;

        let moves: Vec<(usize, usize)> =
            elevation_map
                .positions()
                .enumerate()
                .fold(Vec::new(), |acc, arg| {
                    let mut next_acc = Vec::new();
                    let (index, position) = arg;

                    // UP
                    if position.y > 0 {
                        next_acc.push((index, position.x + (position.y - 1) * columns));
                    }

                    // DOWN
                    if position.y < rows - 1 {
                        next_acc.push((index, position.x + (position.y + 1) * columns));
                    }

                    // LEFT
                    if position.x > 0 {
                        next_acc.push((index, (position.x - 1) + position.y * columns));
                    }

                    // RIGHT
                    if position.x < columns - 1 {
                        next_acc.push((index, (position.x + 1) + position.y * columns));
                    }

                    [acc, next_acc].concat()
                });

        for (from_index, to_index) in moves {
            elevation_map.add_move(from_index, to_index);
        }

        Ok(elevation_map)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SolutionError {
    Unknown,
}

impl std::fmt::Display for SolutionError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => panic!("Unknown error"),
        }
    }
}

impl std::error::Error for SolutionError {}

pub fn solve_first(input: &String) -> Result<String, SolutionError> {
    let elevation_map = input.parse::<ElevationMap>()?;
    let start = elevation_map.get_start();
    let end = elevation_map.get_end();
    let result = elevation_map
        .find_shortest_path(&start, &end)
        .ok_or(SolutionError::Unknown)?;
    Ok(result.len().to_string())
}

pub fn solve_second(input: &String) -> Result<String, SolutionError> {
    let elevation_map = input.parse::<ElevationMap>()?;
    let result = elevation_map
        .positions()
        .enumerate()
        .filter(|index_position| index_position.1.elevation == 'a' as i32)
        .map(|start| {
            let end = elevation_map.get_end();
            elevation_map.find_shortest_path(&start.0, &end)
        })
        .flatten()
        .map(|route| route.len())
        .min()
        .ok_or(SolutionError::Unknown)?;

    Ok(result.to_string())
}
