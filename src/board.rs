use std::fmt;

use bitflags::bitflags;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    value: Option<Digit>,
    candidates: CellMask,
}

bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellMask: u16 {
        const Candidate1 = 0b000000001;
        const Candidate2 = 0b000000010;
        const Candidate3 = 0b000000100;
        const Candidate4 = 0b000001000;
        const Candidate5 = 0b000010000;
        const Candidate6 = 0b000100000;
        const Candidate7 = 0b001000000;
        const Candidate8 = 0b010000000;
        const Candidate9 = 0b100000000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(pub u8);

impl Digit {
    pub fn new(value: u8) -> Self {
        if value < 1 || value > 9 {
            panic!("Digit must be between 1 and 9");
        }
        Digit(value)
    }
}

impl Board {
    pub fn empty() -> Self {
        Board {
            cells: [[Cell {
                value: None,
                candidates: CellMask::empty(),
            }; 9]; 9],
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, digit: Digit) {
        if row >= 9 || col >= 9 {
            panic!("Row and column must be between 0 and 8");
        }
        self.cells[row][col].value = Some(digit);
        self.cells[row][col].candidates = CellMask::empty();
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Digit> {
        if row >= 9 || col >= 9 {
            panic!("Row and column must be between 0 and 8");
        }
        self.cells[row][col].value
    }

    pub fn set_candidates(&mut self, row: usize, col: usize, candidates: CellMask) {
        if row >= 9 || col >= 9 {
            panic!("Row and column must be between 0 and 8");
        }
        self.cells[row][col].candidates = candidates;
    }

    pub fn get_candidates(&self, row: usize, col: usize) -> CellMask {
        if row >= 9 || col >= 9 {
            panic!("Row and column must be between 0 and 8");
        }
        self.cells[row][col].candidates
    }

    pub fn is_solved(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[row][col].value.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn from_string(s: &str) -> Self {
        let mut board = Board::empty();
        for (i, c) in s.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            if c >= '1' && c <= '9' {
                board.set_cell(row, col, Digit::new(c as u8 - b'0'));
            }
        }
        board
    }

    pub fn is_valid(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if let Some(digit) = self.cells[row][col].value {
                    // Check row
                    for c in 0..9 {
                        if c != col && self.cells[row][c].value == Some(digit) {
                            return false;
                        }
                    }
                    // Check column
                    for r in 0..9 {
                        if r != row && self.cells[r][col].value == Some(digit) {
                            return false;
                        }
                    }
                    // Check 3x3 box
                    let box_row_start = (row / 3) * 3;
                    let box_col_start = (col / 3) * 3;
                    for r in box_row_start..box_row_start + 3 {
                        for c in box_col_start..box_col_start + 3 {
                            if (r != row || c != col) && self.cells[r][c].value == Some(digit) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        // Placeholder for solving algorithm
        // This should implement a backtracking algorithm or other solving techniques
        false
    }

    pub fn generate(&mut self) {
        // Placeholder for puzzle generation algorithm
        // This should create a valid Sudoku puzzle with a unique solution
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..9 {
            for col in 0..9 {
                match self.cells[row][col].value {
                    Some(digit) => write!(f, "{} ", digit.0)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
