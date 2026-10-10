//! Library for learning

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use crate::sudoku::Sudoku;

pub fn run_threaded(file_path: &str, pool_size: usize) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(file_path)?;
    let lines: Vec<String> = file.lines().map(|l| String::from(l)).collect();
    let queue = Arc::new(Mutex::new(lines));
    let mut handles = Vec::new();
    for i in 0..pool_size {
        let data = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let mut count = 0;
            loop {
                let queue_result = {
                    let mut q = data.lock().expect("Poisoned!!");
                    q.pop()
                };
                let Some(line) = queue_result else {
                    return count;
                };
                let Some(puzzle) = line.split(':').next() else {
                    continue;
                };
                let Ok(s) = Sudoku::from_str(puzzle) else {
                    eprintln!("Bad line read.");
                    continue;
                };
                match s.solve() {
                    Some(solution) => println!("{i} -> Solved with solution: {solution}"),
                    None => println!("{i} -> No solution for: {line}"),
                };
                count += 1;
            }
        });
        handles.push((i, handle));
    }
    for (i, h) in handles {
        println!("Thread {i} Count: {}", h.join().unwrap())
    }
    Ok(())
}
