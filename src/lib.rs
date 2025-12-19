use anyhow::anyhow;
use std::{collections::HashSet, fmt::Display};

impl Board {
    pub fn solve(&self) -> Result<Self, SolveError> {
        let mut solved = self.clone().is_valid()?;
        while !solved.is_solved() {
            solved = solved.fill_candidates()?;
            println!("{solved}");
        }
        Ok(solved)
        // 0. validate that the initial board is correct
        // 1. identify candidates for each empty cell
        // 2. Put values where there's only one candidate
        // 3. repeat 1-2 until no change is being made
        // 4. output fully solved as Ok or partially solved as Err
    }
    pub fn is_valid(self) -> Result<Self, SolveError> {
        for pos in Value::iter() {
            match ValueSet::from_present_in(self.row(pos).iter()) {
                Some(_) => (),
                None => return Err(SolveError::InvalidInitial(self)),
            };
            match ValueSet::from_present_in(self.col(pos)) {
                Some(_) => (),
                None => return Err(SolveError::InvalidInitial(self)),
            };
            match ValueSet::from_present_in(self.block(pos)) {
                Some(_) => (),
                None => return Err(SolveError::InvalidInitial(self)),
            };
        }
        Ok(self)
    }

    fn is_solved(&self) -> bool {
        self.board
            .iter()
            .flatten()
            .all(|cell| matches!(cell, Cell::Solved(_) | Cell::Filled(_)))
    }

    fn from_cells(cells: [[Cell; 9]; 9]) -> Result<Self, SolveError> {
        Self { board: cells }.is_valid()
    }

    fn fill_candidates(mut self) -> Result<Self, SolveError> {
        // fill for rows
        for row in Value::iter() {
            let candidate_set = ValueSet::from_cells(self.row(row).iter());
            self.row_mut(row).iter_mut().for_each(|cell| {
                match cell {
                    Cell::Empty => *cell = Cell::Candidates(candidate_set.clone()),
                    Cell::Candidates(cell_cands) => {
                        *cell = Cell::Candidates(cell_cands.intersect(&candidate_set))
                    }
                    _ => (),
                };
            });
        }
        // then narrow sets with cols
        for col in Value::iter() {
            let candidate_set = ValueSet::from_cells(self.col(col));
            self.col_mut(col).for_each(|cell| {
                match cell {
                    Cell::Empty => *cell = Cell::Candidates(candidate_set.clone()),
                    Cell::Candidates(cell_cands) => {
                        *cell = Cell::Candidates(cell_cands.intersect(&candidate_set))
                    }
                    _ => (),
                };
            });
        }
        // and then with blocks

        for block in Value::iter() {
            let candidate_set = ValueSet::from_cells(self.block(block));
            self.block_mut(block).for_each(|cell| {
                match cell {
                    Cell::Empty => *cell = Cell::Candidates(candidate_set.clone()),
                    Cell::Candidates(cell_cands) => {
                        *cell = Cell::Candidates(cell_cands.intersect(&candidate_set))
                    }
                    _ => (),
                };
            });
        }
        // iterate over the board and turn single candidates into solved one
        let mut solved_count = 0;
        for cell in self.board.iter_mut().flatten() {
            *cell = match cell.clone() {
                Cell::Candidates(value_set) if value_set.values.len() == 1 => {
                    solved_count += 1;
                    Cell::Solved(value_set.values.into_iter().next().unwrap())
                }
                non_cand_cell => non_cand_cell,
            };
        }
        if solved_count == 0 {
            return Err(SolveError::UnderSolved(self));
        }
        Ok(self)
    }
}
use derive_more::Display;
#[derive(Error, Debug, Display)]
pub enum SolveError {
    /// Initial input is invalid
    #[display("Invalid state of the board")]
    InvalidInitial(Board),
    /// Solver partially with uncertainties remainind
    #[display("Multiple solutions possible")]
    UnderSolved(Board),
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Value {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueSet {
    values: HashSet<Value>,
}

impl<'a, IterCell: Iterator<Item = &'a Cell>> From<IterCell> for ValueSet {
    fn from(value: IterCell) -> Self {
        ValueSet::from_cells(value)
    }
}

impl ValueSet {
    fn intersect(&self, other: &ValueSet) -> Self {
        // the only possible values are in the intersection
        let values = self.values.intersection(&other.values).cloned().collect();
        Self { values }
    }
    // Returns None if there are duplicates
    fn from_present_in<'a>(cells: impl Iterator<Item = &'a Cell>) -> Option<Self> {
        let mut present = HashSet::new();
        for cell in cells {
            let value = match cell {
                Cell::Empty => continue,
                Cell::Filled(value) => value.clone(),
                Cell::Candidates(_) => continue,
                Cell::Solved(value) => value.clone(),
            };

            let inserted = present.insert(value);
            if !inserted {
                return None;
            }
        }
        Some(ValueSet { values: present })
    }

    pub fn from_cells<'a>(cells: impl Iterator<Item = &'a Cell>) -> Self {
        let mut present = HashSet::new();
        for cell in cells {
            if let Cell::Filled(value) | Cell::Solved(value) = cell {
                present.insert(*value);
            }
        }

        let all_values = [
            Value::One,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
        ];

        let mut values = HashSet::new();
        for value in all_values {
            if !present.contains(&value) {
                values.insert(value);
            }
        }

        Self { values }
    }
}

impl Board {
    pub fn row(&self, pos: Value) -> &[Cell; 9] {
        self.board.get(pos as usize - 1).expect("Index in 0..9")
    }
    pub fn row_mut(&mut self, pos: Value) -> &mut [Cell; 9] {
        self.board.get_mut(pos as usize - 1).expect("Index in 0..9")
    }

    pub fn col(&self, pos: Value) -> impl Iterator<Item = &Cell> {
        self.board
            .iter()
            .flatten()
            .skip(pos as usize - 1)
            .step_by(9)
    }

    pub fn col_mut(&mut self, pos: Value) -> impl Iterator<Item = &mut Cell> {
        self.board
            .iter_mut()
            .flatten()
            .skip(pos as usize - 1)
            .step_by(9)
    }

    fn block_mut(&mut self, pos: Value) -> impl Iterator<Item = &mut Cell> {
        Board::block_from(self.board.iter_mut(), pos)
    }

    fn block_from<RowIter, CellItem>(
        rows: impl Iterator<Item = RowIter>,
        pos: Value,
    ) -> impl Iterator<Item = CellItem>
    where
        RowIter: IntoIterator<Item = CellItem>,
    {
        let block_idx = pos as usize - 1;
        let block_row = block_idx / 3;
        let block_col = block_idx % 3;
        let start_row = block_row * 3;
        let start_col = block_col * 3;

        rows.skip(start_row)
            .take(3)
            .flat_map(move |row| row.into_iter().skip(start_col).take(3))
    }

    /// 3x3 block indexed in row-major order, output is also in row-major order
    pub fn block(&self, pos: Value) -> impl Iterator<Item = &Cell> {
        let block_idx = pos as usize - 1;
        let block_row = block_idx / 3;
        let block_col = block_idx % 3;
        let start_row = block_row * 3;
        let start_col = block_col * 3;

        self.board
            .iter()
            .skip(start_row)
            .take(3)
            .flat_map(move |row| row.iter().skip(start_col).take(3))
    }
}

impl TryFrom<u8> for Value {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Value::One),
            2 => Ok(Value::Two),
            3 => Ok(Value::Three),
            4 => Ok(Value::Four),
            5 => Ok(Value::Five),
            6 => Ok(Value::Six),
            7 => Ok(Value::Seven),
            8 => Ok(Value::Eight),
            9 => Ok(Value::Nine),
            _ => Err(anyhow!("Number is out of range")),
        }
    }

    type Error = anyhow::Error;
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            d if d.is_digit(10) => {
                let digit: u8 = d.to_digit(10).unwrap().try_into()?;
                let value: Value = digit.try_into()?;
                Ok(Cell::Filled(value))
            }
            ch => Err(anyhow!("Unexpected character {ch}")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Filled(Value),
    Candidates(ValueSet),
    Solved(Value),
}

use derive_more::Error;

#[derive(Debug, Clone, Error)]
pub struct Board {
    board: [[Cell; 9]; 9],
}

impl std::str::FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let mut rows = Vec::with_capacity(9);
        for line in input.lines().take(9) {
            let row: Vec<Cell> = line
                .chars()
                .filter(|ch| ch != &' ')
                .flat_map(|ch| Cell::try_from(ch))
                .collect();
            let row: [Cell; 9] = <[Cell; 9]>::try_from(row).map_err(|err| anyhow!("{err:?}"))?;
            rows.push(row);
        }
        let board: [[Cell; 9]; 9] = rows.try_into().map_err(|err| anyhow!("{err:?}"))?;
        let board = Self::from_cells(board)?;
        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_string(board: &Board) -> String {
            let mut result = String::new();
            let board = &board.board;
            for row in board.iter() {
                for cell in row.iter() {
                    let ch = match cell {
                        Cell::Empty | Cell::Candidates(_) => '.',
                        Cell::Filled(value) | Cell::Solved(value) => {
                            char::from_digit(*value as u32, 10).expect("digit in 1..=9")
                        }
                    };
                    result.push(ch);
                    result.push(' ');
                }
                result.push('\n');
            }
            result
        }
        write!(f, "{}", to_string(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_try_from_valid_u8() {
        assert!(matches!(Value::try_from(1), Ok(Value::One)));
        assert!(matches!(Value::try_from(2), Ok(Value::Two)));
        assert!(matches!(Value::try_from(3), Ok(Value::Three)));
        assert!(matches!(Value::try_from(4), Ok(Value::Four)));
        assert!(matches!(Value::try_from(5), Ok(Value::Five)));
        assert!(matches!(Value::try_from(6), Ok(Value::Six)));
        assert!(matches!(Value::try_from(7), Ok(Value::Seven)));
        assert!(matches!(Value::try_from(8), Ok(Value::Eight)));
        assert!(matches!(Value::try_from(9), Ok(Value::Nine)));
    }

    #[test]
    fn test_value_try_from_invalid_u8() {
        assert!(Value::try_from(0).is_err());
        assert!(Value::try_from(10).is_err());
        assert!(Value::try_from(255).is_err());
    }

    #[test]
    fn test_cell_try_from_empty() {
        let cell = Cell::try_from('.').unwrap();
        assert!(matches!(cell, Cell::Empty));
    }

    #[test]
    fn test_cell_try_from_filled_digits() {
        assert!(matches!(
            Cell::try_from('1').unwrap(),
            Cell::Filled(Value::One)
        ));
        assert!(matches!(
            Cell::try_from('5').unwrap(),
            Cell::Filled(Value::Five)
        ));
        assert!(matches!(
            Cell::try_from('9').unwrap(),
            Cell::Filled(Value::Nine)
        ));
    }

    #[test]
    fn test_cell_try_from_invalid_char() {
        assert!(Cell::try_from('a').is_err());
        assert!(Cell::try_from('!').is_err());
        assert!(Cell::try_from('0').is_err());
    }

    #[test]
    fn test_board_try_from_valid() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .\n\
                     . 9 8 . . . . 6 .\n\
                     8 . . . 6 . . . 3\n\
                     4 . . 8 . 3 . . 1\n\
                     7 . . . 2 . . . 6\n\
                     . 6 . . . . 2 8 .\n\
                     . . . 4 1 9 . . 5\n\
                     . . . . 8 . . 7 9";
        let board = Board::try_from(input);
        assert!(board.is_ok());
    }

    #[test]
    fn test_board_try_from_invalid_too_few_rows() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .";
        let board = Board::try_from(input);
        assert!(board.is_err());
    }

    #[test]
    fn test_board_try_from_invalid_too_few_cols() {
        let input = "5 3 . . 7\n\
                     6 . . 1 9\n\
                     . 9 8 . .\n\
                     8 . . . 6\n\
                     4 . . 8 .\n\
                     7 . . . 2\n\
                     . 6 . . .\n\
                     . . . 4 1\n\
                     . . . . 8";
        let board = Board::try_from(input);
        assert!(board.is_err());
    }

    #[test]
    fn test_board_to_string() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .\n\
                     . 9 8 . . . . 6 .\n\
                     8 . . . 6 . . . 3\n\
                     4 . . 8 . 3 . . 1\n\
                     7 . . . 2 . . . 6\n\
                     . 6 . . . . 2 8 .\n\
                     . . . 4 1 9 . . 5\n\
                     . . . . 8 . . 7 9";
        let board = Board::try_from(input).unwrap();
        let output = board.to_string();

        // Check that output contains expected patterns
        assert!(output.contains("5 3 ."));
        assert!(output.contains("6 . ."));
        // Check we have 9 lines
        assert_eq!(output.lines().count(), 9);
        // Check first line
        assert_eq!(output.lines().next().unwrap(), "5 3 . . 7 . . . . ");
    }

    #[test]
    fn test_board_row() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .\n\
                     . 9 8 . . . . 6 .\n\
                     8 . . . 6 . . . 3\n\
                     4 . . 8 . 3 . . 1\n\
                     7 . . . 2 . . . 6\n\
                     . 6 . . . . 2 8 .\n\
                     . . . 4 1 9 . . 5\n\
                     . . . . 8 . . 7 9";
        let board = Board::try_from(input).unwrap();

        // Test row 1: should be [5,3,.,.,7,.,.,.,.]
        let row1 = board.row(Value::One);
        assert_eq!(row1.len(), 9);
        assert!(matches!(row1[0], Cell::Filled(Value::Five)));
        assert!(matches!(row1[1], Cell::Filled(Value::Three)));
        assert!(matches!(row1[2], Cell::Empty));
        assert!(matches!(row1[4], Cell::Filled(Value::Seven)));

        // Test row 2: should be [6,.,.,1,9,5,.,.,.]
        let row2 = board.row(Value::Two);
        assert_eq!(row2.len(), 9);
        assert!(matches!(row2[0], Cell::Filled(Value::Six)));
        assert!(matches!(row2[1], Cell::Empty));
        assert!(matches!(row2[3], Cell::Filled(Value::One)));
        assert!(matches!(row2[4], Cell::Filled(Value::Nine)));
        assert!(matches!(row2[5], Cell::Filled(Value::Five)));
    }

    #[test]
    fn test_board_col() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .\n\
                     . 9 8 . . . . 6 .\n\
                     8 . . . 6 . . . 3\n\
                     4 . . 8 . 3 . . 1\n\
                     7 . . . 2 . . . 6\n\
                     . 6 . . . . 2 8 .\n\
                     . . . 4 1 9 . . 5\n\
                     . . . . 8 . . 7 9";
        let board = Board::try_from(input).unwrap();

        // Test col 1: should be [5,6,.,8,4,7,.,.,.]
        let col1: Vec<_> = board.col(Value::One).collect();
        assert_eq!(col1.len(), 9);
        assert!(matches!(col1[0], Cell::Filled(Value::Five)));
        assert!(matches!(col1[1], Cell::Filled(Value::Six)));
        assert!(matches!(col1[2], Cell::Empty));
        assert!(matches!(col1[3], Cell::Filled(Value::Eight)));
        assert!(matches!(col1[4], Cell::Filled(Value::Four)));

        // Test col 5: should be [7,9,.,6,.,2,.,1,8]
        let col5: Vec<_> = board.col(Value::Five).collect();
        assert_eq!(col5.len(), 9);
        assert!(matches!(col5[0], Cell::Filled(Value::Seven)));
        assert!(matches!(col5[1], Cell::Filled(Value::Nine)));
        assert!(matches!(col5[2], Cell::Empty));
        assert!(matches!(col5[3], Cell::Filled(Value::Six)));
        assert!(matches!(col5[8], Cell::Filled(Value::Eight)));
    }

    #[test]
    fn test_board_block() {
        let input = "5 3 . . 7 . . . .\n\
                     6 . . 1 9 5 . . .\n\
                     . 9 8 . . . . 6 .\n\
                     8 . . . 6 . . . 3\n\
                     4 . . 8 . 3 . . 1\n\
                     7 . . . 2 . . . 6\n\
                     . 6 . . . . 2 8 .\n\
                     . . . 4 1 9 . . 5\n\
                     . . . . 8 . . 7 9";
        let board = Board::try_from(input).unwrap();

        // Test block 1 (top-left): should be [5,3,.,6,..,.,9,8]
        let block1: Vec<_> = board.block(Value::One).collect();
        assert_eq!(block1.len(), 9);
        assert!(matches!(block1[0], Cell::Filled(Value::Five)));
        assert!(matches!(block1[1], Cell::Filled(Value::Three)));
        assert!(matches!(block1[2], Cell::Empty));
        assert!(matches!(block1[3], Cell::Filled(Value::Six)));

        // Test block 5 (center): should be [.,6,.,8,.,3,.,2,.]
        let block5: Vec<_> = board.block(Value::Five).collect();
        assert_eq!(block5.len(), 9);
        assert!(matches!(block5[0], Cell::Empty));
        assert!(matches!(block5[1], Cell::Filled(Value::Six)));
        assert!(matches!(block5[3], Cell::Filled(Value::Eight)));
        assert!(matches!(block5[5], Cell::Filled(Value::Three)));
    }
}
