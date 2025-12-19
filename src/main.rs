use sudoku_solver::Board;

fn main() -> anyhow::Result<()> {
    let board = include_str!("../board_hard.txt");
    let board: Board = board.parse()?;
    println!("{board}");
    let show_progress = false;
    let solved = board.solve(show_progress)?;
    if !show_progress {
        println!("{solved}");
    }
    Ok(())
}
