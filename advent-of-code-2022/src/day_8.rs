use std::str::FromStr;

#[derive(Debug)]
struct MatrixParseError;

#[derive(Debug)]
struct Matrix {
    column_count: usize,
    row_count: usize,
    value: Vec<u32>,
}

#[derive(Clone, Copy, Debug)]
struct MatrixIndex {
    column: usize,
    row: usize,
}

trait TreehouseMap {
    fn as_scenic_score(&self, origin: MatrixIndex) -> Option<u32>;
    fn as_visibility(&self, origin: MatrixIndex) -> Option<bool>;
    fn visible_from_north_south(&self, origin: MatrixIndex) -> Option<bool>;
    fn visible_from_west_east(&self, origin: MatrixIndex) -> Option<bool>;
}

impl Matrix {
    fn cell(&self, index: MatrixIndex) -> Option<&u32> {
        self.value.get(index.column + index.row * self.column_count)
    }
    fn column(&self, index: usize) -> Option<Vec<&u32>> {
        if self.column_count > index {
            Some(
                self.value
                    .iter()
                    .skip(index)
                    .step_by(self.column_count)
                    .collect(),
            )
        } else {
            None
        }
    }
    // TODO: This should be Iter
    fn indexes(&self) -> Vec<MatrixIndex> {
        let mut result = Vec::new();
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                result.push(MatrixIndex { column, row })
            }
        }
        return result;
    }
    fn row(&self, index: usize) -> Option<Vec<&u32>> {
        if self.row_count > index {
            Some(
                self.value
                    .iter()
                    .skip(index * self.column_count)
                    .take(self.column_count)
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl TreehouseMap for Matrix {
    fn as_scenic_score(&self, origin: MatrixIndex) -> Option<u32> {
        if origin.column == 0 || origin.column >= self.column_count - 1 {
            return Some(0);
        }

        if origin.row == 0 || origin.row >= self.row_count - 1 {
            return Some(0);
        }

        let height = self.cell(origin)?;

        let column = self.column(origin.column)?;
        let row = self.row(origin.row)?;

        // These are reversed as we want to count outwards
        let north = Vec::from_iter(column[0..origin.row].iter().rev().cloned());
        let west = Vec::from_iter(row[0..origin.column].iter().rev().cloned());

        let south = Vec::from_iter(column[origin.row + 1..].iter().cloned());
        let east = Vec::from_iter(row[origin.column + 1..].iter().cloned());

        let score = [north, south, west, east].iter().fold(1, |score, heights| {
            let mut done = false;
            let trees: u32 = heights
                .iter()
                .take_while(|h| {
                    if done {
                        return false;
                    } else if h < &&height {
                        return true;
                    } else {
                        // This one counts but next one will not
                        done = true;
                        return true;
                    }
                })
                .count()
                .try_into()
                .unwrap();
            score * trees
        });

        Some(score)
    }
    fn as_visibility(&self, origin: MatrixIndex) -> Option<bool> {
        if origin.column == 0 || origin.column >= self.column_count - 1 {
            return Some(true);
        }

        if origin.row == 0 || origin.row >= self.row_count - 1 {
            return Some(true);
        }

        let north_south = self.visible_from_north_south(origin.clone());
        let west_east = self.visible_from_west_east(origin.clone());

        match (north_south, west_east) {
            (Some(false), Some(false)) => Some(false),
            (None, _) => None,
            (_, None) => None,
            _ => Some(true),
        }
    }
    fn visible_from_north_south(&self, origin: MatrixIndex) -> Option<bool> {
        if origin.row == 0 || origin.row >= self.row_count - 1 {
            return Some(true);
        }

        let height = self.cell(origin)?;
        let column = self.column(origin.column)?;
        let (north, south) = column.split_at(origin.row);
        let north_max = north.iter().max()?;
        let south_max = south.iter().skip(1).max()?;

        Some(north_max < &height || south_max < &height)
    }
    fn visible_from_west_east(&self, origin: MatrixIndex) -> Option<bool> {
        if origin.column == 0 || origin.column >= self.column_count - 1 {
            return Some(true);
        }

        let height = self.cell(origin)?;
        let row = self.row(origin.row)?;
        let (west, east) = row.split_at(origin.column);
        let west_max = west.iter().max()?;
        let east_max = east.iter().skip(1).max()?;

        Some(west_max < &height || east_max < &height)
    }
}

impl FromStr for Matrix {
    type Err = MatrixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_count: usize = 0;
        let mut column_count: usize = 0;
        let mut rows = Vec::new();

        for line in s.lines() {
            row_count += 1;
            let mut row_column_count: usize = 0;
            for ch in line.chars() {
                row_column_count += 1;
                let item = ch.to_digit(10).ok_or(MatrixParseError)?;
                rows.push(item);
            }
            if column_count != 0 && column_count != row_column_count {
                return Err(MatrixParseError);
            } else {
                column_count = row_column_count;
            }
        }

        Ok(Matrix {
            column_count,
            row_count,
            value: rows,
        })
    }
}

fn solve_first(input: &String) -> String {
    let treehouse_map = input.parse::<Matrix>().unwrap();
    let visibility: i32 = treehouse_map
        .indexes()
        .iter()
        .map(|i| {
            let visible = treehouse_map
                .as_visibility(MatrixIndex {
                    column: i.column,
                    row: i.row,
                })
                .unwrap();
            if visible {
                1
            } else {
                0
            }
        })
        .sum();
    visibility.to_string()
}

fn solve_second(input: &String) -> String {
    let treehouse_map = input.parse::<Matrix>().unwrap();
    let score: u32 = treehouse_map
        .indexes()
        .iter()
        .map(|i| {
            treehouse_map
                .as_scenic_score(MatrixIndex {
                    column: i.column,
                    row: i.row,
                })
                .unwrap()
        })
        .max()
        .unwrap();
    score.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_matrix_row() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let row = matrix.row(0).unwrap();
        assert_eq!(row, vec![&1, &2, &3]);
    }

    #[test]
    fn test_matrix_row_last() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let row = matrix.row(2).unwrap();
        assert_eq!(row, vec![&7, &8, &9]);
    }

    #[test]
    fn test_matrix_row_out_of_bounds() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let row = matrix.row(3);
        assert_eq!(row, None);
    }

    #[test]
    fn test_matrix_column() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let column = matrix.column(0).unwrap();
        assert_eq!(column, vec![&1, &4, &7]);
    }

    #[test]
    fn test_matrix_column_last() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let column = matrix.column(2).unwrap();
        assert_eq!(column, vec![&3, &6, &9]);
    }

    #[test]
    fn test_matrix_column_out_of_bounds() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let column = matrix.column(3);
        assert_eq!(column, None);
    }

    #[test]
    fn test_matrix_cell() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let cell = matrix.cell(MatrixIndex { column: 0, row: 0 }).unwrap();
        assert_eq!(cell, &1);
    }

    #[test]
    fn test_matrix_cell_last() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let cell = matrix.cell(MatrixIndex { column: 2, row: 2 }).unwrap();
        assert_eq!(cell, &9);
    }

    #[test]
    fn test_matrix_cell_out_of_bounds() {
        let matrix = String::from("123\n456\n789").parse::<Matrix>().unwrap();
        let cell_x = matrix.cell(MatrixIndex { column: 3, row: 2 });
        let cell_y = matrix.cell(MatrixIndex { column: 2, row: 3 });
        let cell = matrix.cell(MatrixIndex { column: 3, row: 3 });
        assert_eq!(cell_x, None);
        assert_eq!(cell_y, None);
        assert_eq!(cell, None);
    }

    #[test]
    fn test_treehouse_map_visibility() {
        let matrix = String::from("30373\n25512\n65332\n33549\n35390")
            .parse::<Matrix>()
            .unwrap();
        let result: Vec<i32> = matrix
            .indexes()
            .iter()
            .map(|i| {
                let visible = matrix
                    .as_visibility(MatrixIndex {
                        column: i.column,
                        row: i.row,
                    })
                    .unwrap();
                if visible {
                    1
                } else {
                    0
                }
            })
            .collect();
        assert_eq!(
            result,
            [
                1, 1, 1, 1, 1, //  1
                1, 1, 1, 0, 1, //  2
                1, 1, 0, 1, 1, //  3
                1, 0, 1, 0, 1, //  4
                1, 1, 1, 1, 1 //   5
            ]
        );
    }

    #[test]
    fn test_treehouse_map_scenic_score() {
        let matrix = String::from("30373\n25512\n65332\n33549\n35390")
            .parse::<Matrix>()
            .unwrap();
        let result_a: u32 = matrix
            .as_scenic_score(MatrixIndex { column: 2, row: 1 })
            .unwrap();
        let result_b: u32 = matrix
            .as_scenic_score(MatrixIndex { column: 2, row: 3 })
            .unwrap();
        assert_eq!(result_a, 4);
        assert_eq!(result_b, 8);
    }

    #[test]
    fn test_treehouse_map_scenic_score_full() {
        let matrix = String::from("30373\n25512\n65332\n33549\n35390")
            .parse::<Matrix>()
            .unwrap();
        let result: u32 = matrix
            .indexes()
            .iter()
            .map(|i| {
                matrix
                    .as_scenic_score(MatrixIndex {
                        column: i.column,
                        row: i.row,
                    })
                    .unwrap()
            })
            .max()
            .unwrap();
        assert_eq!(result, 8);
    }
}
