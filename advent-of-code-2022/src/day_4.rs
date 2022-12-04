use std::collections::HashSet;

struct Assignment {
    left: i32,
    right: i32,
}
struct Pair(Assignment, Assignment);

trait Overlap {
    fn is_concealing(&self) -> bool;
    fn is_intersecting(&self) -> bool;
}

impl From<String> for Assignment {
    fn from(s: String) -> Assignment {
        let mut bounds = s.split("-").take(2);
        let left = bounds
            .next()
            .expect("no left bound")
            .parse::<i32>()
            .expect("no valid left bound");
        let right = bounds
            .next()
            .expect("no right bound")
            .parse::<i32>()
            .expect("no valid right bound");

        Assignment { left, right }
    }
}

impl From<&String> for Pair {
    fn from(s: &String) -> Pair {
        let mut assignments = s
            .split(",")
            .take(2)
            .map(|a| Assignment::from(a.to_string()));
        let assignment_1 = assignments.next().expect("no left assignment found");
        let assignment_2 = assignments.next().expect("no right assignment found");

        Pair(assignment_1, assignment_2)
    }
}

impl Overlap for Pair {
    fn is_intersecting(&self) -> bool {
        let assignment_a: HashSet<i32> = (self.0.left..=self.0.right).collect();
        let assignment_b: HashSet<i32> = (self.1.left..=self.1.right).collect();

        assignment_a.intersection(&assignment_b).count().gt(&0)
    }
    fn is_concealing(&self) -> bool {
        let assignment_a: HashSet<i32> = (self.0.left..=self.0.right).collect();
        let assignment_b: HashSet<i32> = (self.1.left..=self.1.right).collect();

        assignment_a.is_subset(&assignment_b) || assignment_a.is_superset(&assignment_b)
    }
}

fn parse_input(data: Vec<String>) -> Vec<Pair> {
    data.iter().map(|datum| Pair::from(datum)).collect()
}

fn solve_first(input: &Vec<Pair>) -> String {
    input
        .iter()
        .fold(0, |c, p| if p.is_concealing() { c + 1 } else { c })
        .to_string()
}

fn solve_second(input: &Vec<Pair>) -> String {
    input
        .iter()
        .fold(0, |c, p| if p.is_intersecting() { c + 1 } else { c })
        .to_string()
}

pub fn solve(input: Vec<String>) -> (String, String) {
    let data = parse_input(input);

    (solve_first(&data), solve_second(&data))
}
