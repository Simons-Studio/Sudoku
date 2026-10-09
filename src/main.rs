use std::{
    error::Error,
    fs, process,
    sync::{Arc, Mutex},
    thread,
};

use crate::sudoku::Sudoku;

pub mod sudoku;

fn main() {
    // if let Err(e) = run() {
    //     eprintln!("Error parsing puzzles: {e}");
    //     process::exit(1);
    // }

    if let Err(e) = run_threaded("problems.txt", 4) {
        eprint!("Error parsing puzzles: {e}");
        process::exit(1);
    }
}

fn _run() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("test_puzzles")?;
    for (i, line) in file.lines().enumerate() {
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

fn run_threaded(file_path: &str, pool_size: u32) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(file_path)?;
    let lines: Vec<String> = file.lines().map(|l| String::from(l)).collect();
    let queue = Arc::new(Mutex::new(lines));
    let threads = (0..pool_size).map(|i| {
        let thread_q = Arc::clone(&queue);
        thread::spawn(move || {
            let mut count = 0;
            loop {
                let opt_line = {
                    let mut q = thread_q.lock().expect("Thread can't lock");
                    let result = q.pop();
                    drop(q);
                    result
                };
                let Some(line) = opt_line else {
                    break count;
                };
                let Some(puzzle) = line.split(":").next() else {
                    continue;
                };
                let Ok(s) = Sudoku::from_str(&puzzle) else {
                    continue;
                };
                match s.solve() {
                    Some(solution) => println!("T{}> Solved with solution: {solution}", i),
                    None => println!("T{}> No solution to this puzzle: {line}", i),
                }
                count += 1;
            }
        })
    });
    for r in threads {
        let t_join = r.join().expect("Thread failed.");
        println!("Solve count {t_join}.");
    }
    Ok(())
}
