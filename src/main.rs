use clap::Parser;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use sudoku_solver::board::Board;

#[derive(Parser)]
pub struct Args {
    #[arg(default_value_t = false, short = 'p', help = "show progress")]
    pub show_progress: bool,
    #[arg(help = "path to text file with board")]
    pub board_path: PathBuf,
}

fn read_from_file(path: &Path) -> anyhow::Result<Board> {
    let board_str = {
        let mut board_str = String::new();
        File::open(path)?.read_to_string(&mut board_str)?;
        board_str
    };
    board_str.parse()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let board = read_from_file(&args.board_path)?;
    println!("{board}");
    let solved = board.solve(args.show_progress)?;
    println!("{solved}");
    Ok(())
}
