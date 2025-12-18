fn main() -> anyhow::Result<()> {
    let board = include_str!("../board.txt");
    let board: sudoku::Board = board.try_into()?;
    println!("{}", board.to_string());
    Ok(())
}

mod sudoku {

    use anyhow::anyhow;

    #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug)]
    pub enum Cell {
        Empty,
        Filled(Value),
    }

    #[derive(Debug)]
    pub struct Board {
        board: [[Cell; 9]; 9],
    }

    impl TryFrom<&str> for Board {
        type Error = anyhow::Error;

        fn try_from(input: &str) -> anyhow::Result<Self> {
            let mut rows = Vec::with_capacity(9);
            for line in input.lines().take(9) {
                let row: Vec<Cell> = line
                    .chars()
                    .filter(|ch| ch != &' ')
                    .flat_map(|ch| Cell::try_from(ch))
                    .collect();
                let row: [Cell; 9] =
                    <[Cell; 9]>::try_from(row).map_err(|err| anyhow!("{err:?}"))?;
                rows.push(row);
            }
            let board: [[Cell; 9]; 9] = rows.try_into().map_err(|err| anyhow!("{err:?}"))?;
            Ok(Self { board })
        }
    }

    impl ToString for Board {
        fn to_string(&self) -> String {
            let mut result = String::new();
            let board = &self.board;
            for row in board.iter() {
                for cell in row.iter() {
                    let ch = match cell {
                        Cell::Empty => '.',
                        Cell::Filled(value) => {
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
    }
}
