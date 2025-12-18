use sudoku_solver::Board;

fn main() -> anyhow::Result<()> {
    let board = include_str!("../board.txt");
    let board: Board = board.try_into()?;
    println!("{}", board.to_string());
    let solved = board.solve();
    println!("{}", solved.to_string());
    Ok(())
}
