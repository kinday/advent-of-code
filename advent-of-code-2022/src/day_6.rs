use std::collections::{HashSet, VecDeque};

const START_OF_PACKET_MARKER_LENGTH: usize = 4;
const START_OF_MESSAGE_MARKER_LENGTH: usize = 14;

fn find_signal(length: usize, s: &str) -> Option<usize> {
    let mut char_set = HashSet::with_capacity(4);
    let mut char_buffer = VecDeque::with_capacity(5);
    for x in s.chars().enumerate() {
        char_buffer.push_back(x);
        if char_buffer.len() > length {
            char_buffer.pop_front();
            char_set.clear();
            if char_buffer.iter().map(|x| char_set.insert(x.1)).all(|x| x) {
                return Some(x.0 + 1);
            }
        }
    }

    None
}

fn solve_first(input: &String) -> String {
    find_signal(START_OF_PACKET_MARKER_LENGTH, input)
        .unwrap()
        .to_string()
}

fn solve_second(input: &String) -> String {
    find_signal(START_OF_MESSAGE_MARKER_LENGTH, input)
        .unwrap()
        .to_string()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}
