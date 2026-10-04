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
    for (i, line) in file.lines().rev().enumerate() {
        let Some(puzzle) = line.split(":").next() else {
            continue;
        };
        let s = Sudoku::from_str(puzzle)?;
        println!("Puzzle {i}:\n{}", s.to_pretty_string());
        match s.solve() {
            Some(solution) => println!("Solved with solution:\n{}", solution.to_pretty_string()),
            None => println!("No solution to this puzzle."),
        }
    }
    Ok(())
}
