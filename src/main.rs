use std::fmt::Display;

#[derive(Copy, Clone)]
struct Cell {
    value: u32,
}

impl Cell {
    /// # Examples
    ///
    /// ```
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
            let value = self.value.ilog2();
            write!(f, "{}", value)
        }
    }
}

fn cells_to_string(cells: &[Cell]) -> String {
    let cell_strs: Vec<String> = cells.iter().map(|c| c.to_string()).collect();
    cell_strs.join(" ")
}

#[derive(Clone)]
struct Sudoku {
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

    pub fn solve(&mut self) {
        loop {
            let Some(index) = self.min_index() else {
                println!("Solved");
                break;
            };
            let min_value = self.cells[index].value;
            if min_value == 0 {
                panic!("Can't solve.")
            }
            println!("{}, {:b}", index, &self.cells[index].value);
            self.reduce(index);
        }
    }

    fn position(index: usize) -> (usize, usize) {
        let col_index = index % 9;
        let row_index = index / 9;
        (row_index, col_index)
    }

    fn reduce(&mut self, index: usize) {
        let (row, col) = Sudoku::position(index);
        self.reduce_row(row, self.read_row(row));
        self.reduce_col(col, self.read_col(col));
        self.reduce_box(row, col, self.read_box(row, col));
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
            self.cells[index] = Cell {
                value: self.cells[index].value & !used_values,
            };
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
            self.cells[index] = Cell {
                value: self.cells[index].value & !used_values,
            }
        }
    }

    fn read_box(&self, row: usize, col: usize) -> u32 {
        let mut used_values = 0;
        let row_start = row / 3 * 3;
        let col_start = col / 3 * 3;
        for r in row_start..row_start + 3 {
            for c in col_start..col_start + 3 {
                let index = r * 9 + c;
                used_values |= self.cells[index].value;
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
                self.cells[index] = Cell {
                    value: self.cells[index].value & !used_values,
                }
            }
        }
    }

    fn min_index(&self) -> Option<usize> {
        let mut i = 0;
        let mut min = 0;
        let mut min_i = None;
        for cell in &self.cells {
            if cell.is_resolved() {
                continue;
            }
            if min == 0 || cell.bit_count() < min {
                min = cell.bit_count();
                min_i = Some(i);
            }
            i += 1
        }
        min_i
    }
}

fn main() {
    let puzzle =
        "467100805912835607085647192296351470708920351531408926073064510624519783159783064";
    let solution =
        "467192835912835647385647192296351478748926351531478926873264519624519783159783264";

    let c = Cell::from_char('0');
    println!("{}", c.value);
    println!("{}", c.to_string());

    let mut s = Sudoku::from_str(puzzle).expect("Bad input");
    println!("{}", s.to_string());
    println!("{}", s.to_pretty_string());
    s.solve();
    println!("{}", s.to_pretty_string());
}
