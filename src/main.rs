fn main() {
    println!("Hello, world!");
}

mod sudoku {
    enum Value {
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
                _ => Err("Out of range"),
            }
        }

        type Error = &'static str;
    }

    enum Cell {
        Empty,
        Filled(Value),
    }

    struct Board {
        board: [[Cell; 9]; 9],
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
