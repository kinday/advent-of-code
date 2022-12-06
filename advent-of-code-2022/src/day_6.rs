use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct StartMarkerParseError;

struct StartMarker {
    value: usize,
}

impl std::str::FromStr for StartMarker {
    type Err = StartMarkerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut char_index = HashMap::new();
        // let mut char_buffer = VecDeque::with_capacity(4);
        // for (i, c) in s.chars().enumerate() {
        //   char_buffer.push_back(c);
        //   char_index.insert(c, i);
        //   char_buffer.iter().map(|c| char_index.get(&c).and_then()).fuse()
        // }
        let mut char_set = HashSet::with_capacity(4);
        let mut char_buffer = VecDeque::with_capacity(5);
        for x in s.chars().enumerate() {
            char_buffer.push_back(x);
            // TODO: Make parametrized
            if char_buffer.len() > 14 {
                char_buffer.pop_front();
                char_set.clear();
                if char_buffer.iter().map(|x| char_set.insert(x.1)).all(|x| x) {
                    return Ok(StartMarker { value: x.0 + 1 });
                }
            }
        }

        Err(StartMarkerParseError)
    }
}

fn solve_first(input: &String) -> String {
    input.parse::<StartMarker>().unwrap().value.to_string()
}

fn solve_second(input: &String) -> String {
    String::from("...")
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}
