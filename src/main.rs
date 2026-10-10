use std::process;

mod queue;
mod run;
mod sudoku;

fn main() {
    if let Err(e) = run::run_threaded("data/puzzles5_forum_hardest_1905_11+", 16) {
        eprint!("Error parsing puzzles: {e}");
        process::exit(1);
    }
}
