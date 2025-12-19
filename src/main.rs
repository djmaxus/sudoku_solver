use sudoku_solver::Board;

fn main() -> anyhow::Result<()> {
    let board = include_str!("../board_hard.txt");
    let board: Board = board.try_into()?;
    println!("{board}");
    board.solve()?;
    Ok(())
}
