type Calories = i32;

#[derive(Debug, Eq, PartialEq)]
pub struct InventoryParseError;

impl std::fmt::Display for InventoryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse inventory")
    }
}

impl std::error::Error for InventoryParseError {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Inventory {
    value: Vec<Calories>,
}

impl std::iter::IntoIterator for Inventory {
    type Item = Calories;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TroupeParseError;

impl std::fmt::Display for TroupeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse troupe")
    }
}

impl std::error::Error for TroupeParseError {}

// "Troupe of elves", courtesy of 93 seconds long research
#[derive(Debug, Eq, PartialEq)]
struct Troupe {
    value: Vec<Inventory>,
}

impl std::str::FromStr for Inventory {
    type Err = InventoryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (results, errors): (Vec<_>, Vec<_>) = s
            .lines()
            .map(|line| line.parse::<i32>())
            .partition(Result::is_ok);

        if !errors.is_empty() {
            return Err(InventoryParseError);
        }

        Ok(Inventory {
            value: results.iter().flatten().cloned().collect(),
        })
    }
}

impl std::str::FromStr for Troupe {
    type Err = TroupeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (results, errors): (Vec<_>, Vec<_>) = s
            .split("\n\n")
            .map(|inventory| inventory.parse::<Inventory>())
            .partition(Result::is_ok);

        if !errors.is_empty() {
            return Err(TroupeParseError);
        }

        Ok(Troupe {
            value: results.iter().flatten().cloned().collect(),
        })
    }
}

impl std::iter::IntoIterator for Troupe {
    type Item = Inventory;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SolutionError {
    InventoryParseError(InventoryParseError),
    TroupeParseError(TroupeParseError),
}

impl std::fmt::Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionError::InventoryParseError(err) => err.fmt(f),
            SolutionError::TroupeParseError(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for SolutionError {}

impl From<InventoryParseError> for SolutionError {
    fn from(error: InventoryParseError) -> Self {
        SolutionError::InventoryParseError(error)
    }
}

impl From<TroupeParseError> for SolutionError {
    fn from(error: TroupeParseError) -> Self {
        SolutionError::TroupeParseError(error)
    }
}

pub fn solve_first(input: &String) -> Result<String, SolutionError> {
    let troupe = input.parse::<Troupe>()?;
    let result: Calories = troupe
        .into_iter()
        .map(|inventory| inventory.into_iter().sum())
        .max()
        .ok_or(InventoryParseError)?;

    Ok(result.to_string())
}

pub fn solve_second(input: &String) -> Result<String, SolutionError> {
    let troupe = input.parse::<Troupe>()?;
    let mut result: Vec<Calories> = troupe
        .into_iter()
        .map(|inventory| inventory.into_iter().sum())
        .collect();

    result.sort();
    result.reverse();
    result.truncate(3);

    Ok(result.iter().sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_from_str_ok() {
        let result = "1000\n2000\n3000".parse::<Inventory>();
        assert_eq!(
            result,
            Ok(Inventory {
                value: vec![1000, 2000, 3000]
            })
        );
    }

    #[test]
    fn test_inventory_from_str_err() {
        let result = "1000\nBOOM\n3000".parse::<Inventory>();
        assert_eq!(result, Err(InventoryParseError));
    }

    #[test]
    fn test_troupe_from_str_ok() {
        let result = "1000\n2000\n3000\n\n4000\n5000".parse::<Troupe>();
        assert_eq!(
            result,
            Ok(Troupe {
                value: vec![
                    Inventory {
                        value: vec![1000, 2000, 3000]
                    },
                    Inventory {
                        value: vec![4000, 5000]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_troupe_from_str_err() {
        let result = "1000\n2000\n3000\n\nBOOM\n5000".parse::<Troupe>();
        assert_eq!(result, Err(TroupeParseError));
    }
}
