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
}
