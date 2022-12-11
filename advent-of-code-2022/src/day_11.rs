use std::str::FromStr;

type ItemRecipient = usize;
type ItemWorryLevel = u128;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ItemCooldown {
    Division,
    Modulo(ItemWorryLevel),
}

#[derive(Debug, Eq, PartialEq)]
struct MonkeyOperationArgParseError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MonkeyOperationArg {
    Literal(ItemWorryLevel),
    Old,
}

impl FromStr for MonkeyOperationArg {
    type Err = MonkeyOperationArgParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(MonkeyOperationArg::Old),
            x => x
                .parse::<ItemWorryLevel>()
                .map(|x| MonkeyOperationArg::Literal(x))
                .map_err(|_| MonkeyOperationArgParseError),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MonkeyOperationParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
enum MonkeyOperation {
    Add(MonkeyOperationArg, MonkeyOperationArg),
    Multiply(MonkeyOperationArg, MonkeyOperationArg),
}

impl FromStr for MonkeyOperation {
    type Err = MonkeyOperationParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim().replace("Operation: new = ", "");
        let mut input = input.split(" ");

        let left_arg = input
            .next()
            .ok_or(MonkeyOperationParseError)?
            .parse::<MonkeyOperationArg>()
            .map_err(|_| MonkeyOperationParseError)?;

        let operator = input.next();

        let right_arg = input
            .next()
            .ok_or(MonkeyOperationParseError)?
            .parse::<MonkeyOperationArg>()
            .map_err(|_| MonkeyOperationParseError)?;

        match operator {
            Some("+") => Ok(MonkeyOperation::Add(left_arg, right_arg)),
            Some("*") => Ok(MonkeyOperation::Multiply(left_arg, right_arg)),
            Some(_) | None => Err(MonkeyOperationParseError),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MonkeyTestParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
struct MonkeyTest(
    ItemWorryLevel, // divisor
    ItemRecipient,  // true_recipient
    ItemRecipient,  // false_recipient
);

impl FromStr for MonkeyTest {
    type Err = MonkeyTestParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.lines().map(|x| x.trim());

        let divisor = input
            .next()
            .map(|x| x.replace("Test: divisible by ", ""))
            .ok_or(MonkeyTestParseError)?
            .parse::<ItemWorryLevel>()
            .map_err(|_| MonkeyTestParseError)?;

        let true_recipient = input
            .next()
            .map(|x| x.replace("If true: throw to monkey ", ""))
            .ok_or(MonkeyTestParseError)?
            .parse::<ItemRecipient>()
            .map_err(|_| MonkeyTestParseError)?;

        let false_recipient = input
            .next()
            .map(|x| x.replace("If false: throw to monkey ", ""))
            .ok_or(MonkeyTestParseError)?
            .parse::<ItemRecipient>()
            .map_err(|_| MonkeyTestParseError)?;

        Ok(MonkeyTest(divisor, true_recipient, false_recipient))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MonkeyParseError {
    InvalidIndex,
    InvalidInput,
    InvalidItem,
    InvalidOperation,
    InvalidTest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Monkey {
    index: ItemRecipient,
    items: Vec<ItemWorryLevel>,
    inspected: u128,
    operation: MonkeyOperation,
    test: MonkeyTest,
}

impl Monkey {
    fn inspect(&mut self, cooldown: &ItemCooldown) -> Vec<(ItemRecipient, ItemWorryLevel)> {
        let result = self
            .items
            .iter()
            .map(|item| self.apply_operation(item, cooldown))
            .clone()
            .map(|item| (self.apply_test(&item), item))
            .collect();

        // All items were passed to other monkeys
        self.items = Vec::new();

        result
    }

    fn apply_operation(&self, value: &ItemWorryLevel, cooldown: &ItemCooldown) -> ItemWorryLevel {
        use MonkeyOperation::*;
        use MonkeyOperationArg::*;

        let next_value = match self.operation {
            Add(left, right) => match (left, right) {
                (Old, Old) => value + value,
                (Literal(literal), Old) | (Old, Literal(literal)) => literal + value,
                _ => unimplemented!(),
            },
            Multiply(left, right) => match (left, right) {
                (Old, Old) => value.pow(2),
                (Literal(literal), Old) | (Old, Literal(literal)) => literal * value,
                _ => unimplemented!(),
            },
        };

        match cooldown {
            ItemCooldown::Division => next_value / 3,
            ItemCooldown::Modulo(modulo) => next_value % modulo,
        }
    }

    fn apply_test(&self, item: &ItemWorryLevel) -> ItemRecipient {
        if item % self.test.0.clone() == 0 {
            self.test.1
        } else {
            self.test.2
        }
    }
}

impl FromStr for Monkey {
    type Err = MonkeyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let index_line = lines.next().ok_or(MonkeyParseError::InvalidInput)?;
        let index = index_line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<ItemRecipient>()
            .map_err(|_| MonkeyParseError::InvalidIndex)?;

        let items_line = lines.next().ok_or(MonkeyParseError::InvalidInput)?;
        let items = items_line
            .trim()
            .replace("Starting items:", "")
            .trim()
            .split(", ")
            .map(|x| x.parse::<ItemWorryLevel>())
            .collect::<Result<Vec<ItemWorryLevel>, _>>()
            .map_err(|_| MonkeyParseError::InvalidItem)?;

        let operation_line = lines.next().ok_or(MonkeyParseError::InvalidOperation)?;
        let operation = operation_line
            .parse::<MonkeyOperation>()
            .map_err(|_| MonkeyParseError::InvalidOperation)?;

        let test_line = lines.fold(String::new(), |result, line| result + line + "\n");
        let test = test_line
            .parse::<MonkeyTest>()
            .map_err(|_| MonkeyParseError::InvalidTest)?;

        Ok(Monkey {
            index,
            inspected: 0,
            items,
            operation,
            test,
        })
    }
}

#[derive(Debug)]
struct GameParseError;

#[derive(Clone, Debug)]
struct Game {
    cooldown: ItemCooldown,
    monkeys: Vec<Monkey>,
}

impl Game {
    fn as_monkey_business(&self) -> u128 {
        let mut sizes: Vec<u128> = self.monkeys.iter().map(|monkey| monkey.inspected).collect();
        sizes.sort();
        sizes.reverse();
        sizes.truncate(2);

        sizes.iter().fold(1, |acc, x| acc * x)
    }

    fn use_modulo_cooldown(&mut self) {
        let modulo = self.monkeys.iter().map(|monkey| monkey.test.0).product();
        self.cooldown = ItemCooldown::Modulo(modulo);
    }
}

impl FromStr for Game {
    type Err = GameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, errors): (Vec<_>, Vec<_>) = s
            .split("\n\n")
            .map(|input| input.parse::<Monkey>())
            .partition(Result::is_ok);

        if errors.is_empty() {
            Ok(Game {
                cooldown: ItemCooldown::Division,
                monkeys: result.iter().flatten().cloned().collect(),
            })
        } else {
            Err(GameParseError)
        }
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        let mut passed_items: Vec<(ItemRecipient, ItemWorryLevel)> = Vec::new();

        for monkey in self.monkeys.iter_mut() {
            let (received_items, next_passed_items): (
                Vec<(ItemRecipient, ItemWorryLevel)>,
                Vec<(ItemRecipient, ItemWorryLevel)>,
            ) = passed_items
                .iter()
                .cloned()
                .partition(|item| item.0 == monkey.index);

            passed_items = next_passed_items;

            for item in received_items {
                monkey.items.push(item.1)
            }

            for passed_item in monkey.inspect(&self.cooldown) {
                monkey.inspected += 1;
                passed_items.push(passed_item);
            }
        }

        for (index, item) in passed_items {
            if let Some(monkey) = self.monkeys.get_mut(index) {
                monkey.items.push(item);
            }
        }

        Some(self.clone())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SolutionError {
    Unknown,
}

impl std::fmt::Display for SolutionError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionError::Unknown => panic!("Unknown error"),
        }
    }
}

impl std::error::Error for SolutionError {}

pub fn solve_first(input: &String) -> Result<String, SolutionError> {
    let rounds = 20;
    let mut game = input.parse::<Game>().unwrap();
    let endgame = game.nth(rounds - 1).ok_or(SolutionError::Unknown)?;
    Ok(endgame.as_monkey_business().to_string())
}

pub fn solve_second(input: &String) -> Result<String, SolutionError> {
    let rounds = 10000;
    let mut game = input.parse::<Game>().unwrap();
    game.use_modulo_cooldown();
    let endgame = game.nth(rounds - 1).ok_or(SolutionError::Unknown)?;
    Ok(endgame.as_monkey_business().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_from_str_item_recipient_invalid_input_err() {
        let result = "".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidInput))
    }

    #[test]
    fn test_monkey_from_str_item_recipient_invalid_index_err() {
        let result = "Monkey X:".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidIndex))
    }

    #[test]
    fn test_monkey_from_str_items_invalid_input_err() {
        let result = "Monkey 0:".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidInput))
    }

    #[test]
    fn test_monkey_from_str_items_invalid_item_err() {
        let result = "Monkey 0:\n  Starting items: X, Y".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidItem))
    }

    #[test]
    fn test_monkey_from_str_operation_invalid_operation_err() {
        let result = "Monkey 0:\n  Starting items: 10, 15".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidOperation))
    }

    #[test]
    fn test_monkey_from_str_test_invalid_test_err() {
        let result =
            "Monkey 0:\n  Starting items: 10, 15\n  Operation: new = old * 19".parse::<Monkey>();
        assert_eq!(result, Err(MonkeyParseError::InvalidTest))
    }
}
