use sudoku_solver::Board;

fn main() -> anyhow::Result<()> {
    let board = include_str!("../board_hard.txt");
    let board: Board = board.parse()?;
    println!("{board}");
    board.solve()?;
    Ok(())
}
