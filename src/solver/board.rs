use std::fmt;
use std::ops::Index;
use crate::solver::board::CellState::{Solved, Unsolved};
use crate::solver::calc::{block_of, Cell, col_of, number_to_mask, row_of};
use crate::errors::{ContradicoryAssignmentError, UnsolvableError};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Possibilities {
    pub mask: u16,
    pub count: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Solved(u8),
    Unsolved(Possibilities),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    pub state: [CellState; 81],
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: [Unsolved(Possibilities {
                count: 9,
                mask: 0b111111111,
            }); 81],
        }
    }

    pub fn from_puzzle(fields: Vec<u8>) -> Result<Self, UnsolvableError> {
        let mut board = Board::new();

        for (i, &given) in fields.iter().enumerate() {
            if given != 0 {
                let mark_err = board.mark(Cell((i) as u8), given - 1).is_err();
                if mark_err {
                    return Err(UnsolvableError {})
                }
            }
        }

        Ok(board)
    }

    pub fn to_1d_string (&self) -> String {
        let mut a = String::new();

        for i in 0..81 {
            let val = self.state[i];
            if let Solved(num) = val {
                a.push_str(&*format!("{}", num + 1));
            } else {
                a.push('0');
            }
        }

        a
    }

    // Returns the first unsolved cell with the least remaining possible values
    // If the board does not contain any unsolved cells,
    pub fn most_certain(&self) -> Option<Cell> {
        let mut most_certain = 255u8;
        let mut most_certain_count = 255u8;

        for i in 0..81 {
            if let Unsolved(possibilities) = self.state[i] {
                if most_certain_count > possibilities.count {
                    most_certain_count = possibilities.count;
                    most_certain = i as u8;
                }
            }
        }

        if most_certain == 255 {
            None
        } else {
            Some(Cell(most_certain))
        }
    }

    pub fn mark(&mut self, cell: Cell, val: u8) -> Result<u8, ContradicoryAssignmentError> {
        assert!(val <= 8);
        let current_val = self.state[cell.0 as usize];
        match current_val {
            Solved(already_marked) => {
                if already_marked != val {
                    return Err(ContradicoryAssignmentError {
                        target: cell,
                        attempted_val: val,
                        solved_val: Some(already_marked),
                    })
                }
            }
            Unsolved(possibilies) => {
                let value_mask = number_to_mask(val);
                if possibilies.mask & value_mask > 0 {
                    let mask_away = value_mask ^ 0xFFFF;
                    self.state[cell.0 as usize] = Solved(val);
                    let a = self.mark_of_cells(mask_away, row_of(cell).into_iter());
                    let b = self.mark_of_cells(mask_away, col_of(cell).into_iter());
                    let c = self.mark_of_cells(mask_away, block_of(cell).into_iter());
                    if a.is_err() || b.is_err() || c.is_err() {
                        return Err(ContradicoryAssignmentError {
                            target: cell,
                            attempted_val: val,
                            solved_val: None,
                        })
                    }
                } else {
                    return Err(ContradicoryAssignmentError {
                        target: cell,
                        attempted_val: val,
                        solved_val: None,
                    })
                }
            }
        }
        Ok(val)
    }

    pub fn mark_of_cells<T: Iterator<Item=Cell>>(&mut self, mask_off: u16, iter: T) -> Result<(), ()> {
        for same_row_cell in iter {
            if let Unsolved(mut row_possibilities) = self.state[same_row_cell.0 as usize] {
                row_possibilities.mask &= mask_off;
                row_possibilities.count = row_possibilities.mask.count_ones() as u8;
                if row_possibilities.count == 0 {
                    return Err(());
                } else {
                    self.state[same_row_cell.0 as usize] = Unsolved(row_possibilities);
                }
            }
        }
        Ok(())
    }
}

impl Index<Cell> for Board {
    type Output = CellState;
    fn index(&self, i: Cell) -> &Self::Output {
        &self.state[i.0 as usize]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..9u8 {
            if y == 3 || y == 6 {
                writeln!(f, "---+---+---")?;
            }
            for x in 0..9u8 {
                if x == 3 || x == 6 {
                    write!(f, "|")?;
                }
                let idx = y * 9 + x;
                let val = self.state[idx as usize];
                if let Solved(num) = val {
                    write!(f, "{}", num + 1)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
