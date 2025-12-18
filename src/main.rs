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
    }
}
