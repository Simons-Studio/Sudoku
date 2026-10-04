//! This is the Sudoku library

use core::panic;
use std::{fmt::Display, thread::sleep, time::Duration};

#[derive(Copy, Clone)]
struct Cell {
    value: u32,
}

impl Cell {
    /// Translate a char to a binary representation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cell = Cell::from_char('3');
    /// assert_eq!(cell.value, 8);
    ///
    /// let cell = Cell::from_char('.');
    /// assert_eq!(cell.value, 0);
    /// ```
    fn from_char(c: char) -> Cell {
        if c.is_ascii() && c.is_numeric() {
            let value = c.to_digit(10).unwrap();
            if value < 1 || value > 9 {
                return Cell { value: 0b111111111 };
            };
            Cell {
                value: 1 << (value - 1),
            }
        } else {
            Cell { value: 0b111111111 }
        }
    }

    fn is_resolved(&self) -> bool {
        self.value > 0 && (self.value & (self.value - 1)) == 0
    }

    fn bit_count(&self) -> u32 {
        let mut test = self.value;
        let mut count = 0;
        while test != 0 {
            test = test & (test - 1);
            count += 1
        }
        count
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bit_count() != 1 {
            write!(f, ".")
        } else {
            let value = self.value.ilog2() + 1;
            write!(f, "{}", value)
        }
    }
}

fn cells_to_string(cells: &[Cell]) -> String {
    let cell_strs: Vec<String> = cells.iter().map(|c| c.to_string()).collect();
    cell_strs.join(" ")
}

struct Step {
    index: usize,
    old_state: Cell,
    new_state: Cell,
    other_choices: u32,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.other_choices > 0 {
            "Choice"
        } else {
            "Single"
        };
        write!(
            f,
            "{prefix} {:02}: {:9b} -> {}",
            self.index,
            self.other_choices | self.new_state.value,
            self.new_state
        )
    }
}

impl Step {
    fn is_choice(&self) -> bool {
        self.other_choices > 0
    }

    fn next_choice(self) -> Option<Self> {
        if self.other_choices == 0 {
            return None;
        }
        let new_choice = self.other_choices & !(self.other_choices - 1);
        Some(Step {
            index: self.index,
            old_state: self.old_state,
            new_state: Cell { value: new_choice },
            other_choices: self.other_choices & !new_choice,
        })
    }
}

struct StepReturn {
    is_solution: Option<bool>,
    message: String,
}

#[derive(Clone)]
pub struct Sudoku {
    cells: Vec<Cell>,
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell_strs: Vec<String> = self.cells.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", cell_strs.join(""))
    }
}

impl Sudoku {
    pub fn from_str(s: &str) -> Result<Sudoku, &'static str> {
        let raw_cells: Vec<Cell> = s.chars().map(|c| Cell::from_char(c)).collect();
        if raw_cells.len() != 81 {
            Err("input string is not 81 characters long.")
        } else {
            Ok(Sudoku { cells: raw_cells })
        }
    }

    pub fn to_pretty_string(&self) -> String {
        let box_rows: Vec<String> = self
            .cells
            .chunks(27)
            .map(|box_row| {
                let rows: Vec<String> = box_row
                    .chunks(9)
                    .map(|row| {
                        format!(
                            "{}|{}|{}\n",
                            cells_to_string(&row[0..3]),
                            cells_to_string(&row[3..6]),
                            cells_to_string(&row[6..9])
                        )
                    })
                    .collect();
                rows.join("")
            })
            .collect();
        box_rows.join("-----+-----+-----\n")
    }

    pub fn pretty_solve(mut self) -> Option<Self> {
        self.initialise_options();
        let mut steps: Vec<Step> = Vec::new();
        loop {
            let step = self.solve_step(&mut steps);
            let Some(solution) = step.is_solution else {
                println!("{}\n{}", step.message, self.to_pretty_string());
                sleep(Duration::from_millis(250));
                continue;
            };
            if solution {
                break Some(self);
            } else {
                break None;
            }
        }
    }

    pub fn solve(mut self) -> Option<Self> {
        self.initialise_options();
        let mut steps: Vec<Step> = Vec::new();
        loop {
            let step = self.solve_step(&mut steps);
            let Some(solution) = step.is_solution else {
                continue;
            };
            if solution {
                break Some(self);
            } else {
                break None;
            }
        }
    }

    fn solve_step(&mut self, mut steps: &mut Vec<Step>) -> StepReturn {
        let Some(index) = self.min_index() else {
            println!("Solved!\n{}", self.to_pretty_string());
            return StepReturn {
                is_solution: Some(true),
                message: String::from("Solved"),
            };
        };
        let old_state = self.cells[index];
        if old_state.value == 0 {
            panic!("Invalid Puzzle.")
        }
        let options = Cell {
            value: self.get_options(index),
        };
        if options.bit_count() > 0 {
            let choice = options.value & !(options.value - 1);
            let new_state = Cell { value: choice };
            let step = Step {
                index,
                old_state,
                new_state,
                other_choices: options.value & !choice,
            };
            let message = format!("+{:04} | {step}", steps.len());
            steps.push(step);
            self.cells[index] = new_state;
            StepReturn {
                is_solution: None,
                message,
            }
        } else {
            // Step back up
            match self.step_back(&mut steps) {
                Some(num) => StepReturn {
                    is_solution: None,
                    message: format!("-{num:04} | {}", steps.last().unwrap()),
                },
                None => StepReturn {
                    is_solution: Some(false),
                    message: String::from("Stepped all the way back with no solution."),
                },
            }
        }
    }

    fn step_back(&mut self, steps: &mut Vec<Step>) -> Option<u32> {
        let mut step_count = 0;
        loop {
            let Some(step) = steps.pop() else {
                return None;
            };
            if step.is_choice() {
                let next_step = step.next_choice().unwrap();
                self.cells[next_step.index] = next_step.new_state;
                steps.push(next_step);
                return Some(step_count);
            }
            self.cells[step.index] = step.old_state;
            step_count += 1;
        }
    }

    fn initialise_options(&mut self) {
        for i in 0..9 {
            self.reduce_row(i, self.read_row(i));
            self.reduce_col(i, self.read_col(i));
            let box_row = i / 3 * 3;
            let box_col = i % 3 * 3;
            self.reduce_box(box_row, box_col, self.read_box(box_row, box_col));
        }
    }

    fn position(index: usize) -> (usize, usize) {
        let col_index = index % 9;
        let row_index = index / 9;
        (row_index, col_index)
    }

    fn get_options(&self, index: usize) -> u32 {
        let (row, col) = Sudoku::position(index);
        let used_row = self.read_row(row);
        let used_col = self.read_col(col);
        let used_box = self.read_box(row, col);
        0b111111111 & !(used_row | used_col | used_box)
    }

    fn read_col(&self, col: usize) -> u32 {
        let used_values = self.cells[col..].iter().step_by(9).fold(0, |acc, cell| {
            if cell.is_resolved() {
                acc | cell.value
            } else {
                acc
            }
        });
        used_values
    }

    fn reduce_col(&mut self, col: usize, used_values: u32) {
        for row in 0..9 {
            let index = row * 9 + col;
            if !self.cells[index].is_resolved() {
                self.cells[index].value &= !used_values;
            }
        }
    }

    fn read_row(&self, row: usize) -> u32 {
        let used_values = self.cells[9 * row..9 * (row + 1)]
            .iter()
            .fold(0, |acc, cell| {
                if cell.is_resolved() {
                    acc | cell.value
                } else {
                    acc
                }
            });
        used_values
    }

    fn reduce_row(&mut self, row: usize, used_values: u32) {
        let start = row * 9;
        for index in start..start + 9 {
            if !self.cells[index].is_resolved() {
                self.cells[index].value &= !used_values;
            }
        }
    }

    fn read_box(&self, row: usize, col: usize) -> u32 {
        let mut used_values = 0;
        let row_start = row / 3 * 3;
        let col_start = col / 3 * 3;
        for r in row_start..row_start + 3 {
            for c in col_start..col_start + 3 {
                let cell = &self.cells[r * 9 + c];
                if cell.is_resolved() {
                    used_values |= cell.value;
                }
            }
        }
        used_values
    }

    fn reduce_box(&mut self, row: usize, col: usize, used_values: u32) {
        let row_start = row / 3 * 3;
        let col_start = col / 3 * 3;
        for r in row_start..row_start + 3 {
            for c in col_start..col_start + 3 {
                let index = r * 9 + c;
                if !self.cells[index].is_resolved() {
                    self.cells[index].value &= !used_values;
                }
            }
        }
    }

    fn min_index(&self) -> Option<usize> {
        let mut min = 1024;
        let mut min_i = None;
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_resolved() {
                continue;
            }
            let count = cell.bit_count();
            if count < min {
                min = count;
                min_i = Some(i);
            }
        }
        min_i
    }
}

#[cfg(test)]
mod test {
    use crate::sudoku::{Cell, Sudoku};

    #[test]
    fn cell_value_test() {
        assert_eq!(Cell::from_char('c').value, 511);
        assert_eq!(Cell::from_char('3').value, 4);
    }

    #[test]
    fn cell_output_test() {
        assert_eq!(Cell::from_char('5').to_string(), String::from("5"));
        assert_eq!(Cell::from_char('r').to_string(), String::from("."));
    }

    #[test]
    fn sudoku_easy_test() {
        let solution =
            "467192835912835647385647192296351478748926351531478926873264519624519783159783264";
        let s = Sudoku::from_str(
            "467100805912835607085647192296351470708920351531408926073064510624519783159783064",
        )
        .expect("Bad String");
        let solved = s.solve().expect("No solution.");
        assert_eq!(solved.to_string(), String::from(solution));
    }

    #[test]
    fn sudoku_short_string() {
        let s = Sudoku::from_str("test");
        assert!(s.is_err());
    }

    #[test]
    fn sudoku_long_string() {
        let s = Sudoku::from_str(
            "46710080591283560708564719229635147070892035153140892607306451062451978315978306400000",
        );
        assert!(s.is_err());
    }
}
