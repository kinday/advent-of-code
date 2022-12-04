struct Compartment {
    value: String,
}

struct Rucksack(Compartment, Compartment);

struct Rucksacks(Vec<Rucksack>);

struct Group(Compartment, Compartment, Compartment);
struct Groups(Vec<Group>);

const ASCII_LOWERCASE_A: u32 = 97;
const ASCII_LOWERCASE_Z: u32 = 122;
const ASCII_UPPERCASE_A: u32 = 65;
const ASCII_UPPERCASE_Z: u32 = 90;

trait Priority {
    fn as_priority(&self) -> u32;
    fn to_priority(item: char) -> u32 {
        let code = item as u32;

        return match code {
            (ASCII_LOWERCASE_A..=ASCII_LOWERCASE_Z) => code - ASCII_LOWERCASE_A + 1,
            (ASCII_UPPERCASE_A..=ASCII_UPPERCASE_Z) => code - ASCII_UPPERCASE_A + 27,
            _ => 0,
        };
    }
}

impl From<&String> for Compartment {
    fn from(s: &String) -> Self {
        Compartment { value: s.clone() }
    }
}

impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        Compartment {
            value: s.to_string(),
        }
    }
}

impl From<&String> for Rucksack {
    fn from(s: &String) -> Self {
        let length = s.len();
        let (a, b) = s.split_at(length / 2);

        Rucksack(Compartment::from(a), Compartment::from(b))
    }
}

impl Priority for Rucksack {
    fn as_priority(&self) -> u32 {
        for a in self.0.value.chars() {
            for b in self.1.value.chars() {
                if a == b {
                    return Self::to_priority(b);
                }
            }
        }

        panic!("huh?")
    }
}

impl From<&Vec<String>> for Rucksacks {
    fn from(v: &Vec<String>) -> Self {
        v.iter().map(|x| Rucksack::from(x)).collect()
    }
}

impl FromIterator<Rucksack> for Rucksacks {
    fn from_iter<T: IntoIterator<Item = Rucksack>>(iter: T) -> Self {
        let mut v = Vec::new();
        for item in iter {
            v.push(item)
        }

        Rucksacks(v)
    }
}

impl From<&[String]> for Group {
    fn from(s: &[String]) -> Self {
        let mut compartments = s.iter().map(|x| Compartment::from(x));

        Group(
            compartments.next().expect("no first compartment"),
            compartments.next().expect("no second compartment"),
            compartments.next().expect("no third compartment"),
        )
    }
}

impl From<&Vec<String>> for Groups {
    fn from(v: &Vec<String>) -> Self {
        v.chunks(3).map(|x| Group::from(x)).collect()
    }
}

impl FromIterator<Group> for Groups {
    fn from_iter<T: IntoIterator<Item = Group>>(iter: T) -> Self {
        let mut v = Vec::new();
        for item in iter {
            v.push(item)
        }

        Groups(v)
    }
}

impl Priority for Group {
    fn as_priority(&self) -> u32 {
        let mut ab = Vec::new();
        for a in self.0.value.chars() {
            for b in self.1.value.chars() {
                if a == b {
                    ab.push(a);
                }
            }
        }

        for c in self.2.value.chars() {
            if ab.contains(&c) {
                return Self::to_priority(c);
            }
        }

        panic!("huh?")
    }
}

fn solve_first(input: &Vec<String>) -> String {
    Rucksacks::from(input)
        .0
        .iter()
        .fold(0, |sum, x| sum + x.as_priority())
        .to_string()
}

fn solve_second(input: &Vec<String>) -> String {
    Groups::from(input)
        .0
        .iter()
        .fold(0, |sum, x| sum + x.as_priority())
        .to_string()
}

pub fn solve(input: Vec<String>) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}
