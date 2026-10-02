use std::{error::Error, fs, process};

use crate::sudoku::Sudoku;

pub mod sudoku;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error parsing puzzles: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("test_puzzles")?;
    for line in file.lines() {
        let Some(puzzle) = line.split(":").next() else {
            continue;
        };
        let mut s = Sudoku::from_str(puzzle)?;
        println!("Puzzle:\n{}", s.to_pretty_string());
        let steps = s.solve();
        println!("Solution in {steps} steps:\n{}", s.to_pretty_string());
    }
    Ok(())
}
