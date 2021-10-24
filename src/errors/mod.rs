use std::error::Error;
use std::fmt;
use crate::solver::calc::Cell;

#[derive(Debug, Clone)]
pub struct ContradicoryAssignmentError {
    pub target: Cell,
    pub attempted_val: u8,
    pub solved_val: Option<u8>,
}

impl Error for ContradicoryAssignmentError {}

impl fmt::Display for ContradicoryAssignmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(solved) = self.solved_val {
            write!(f, "Attempt to write {} to cell {}, but it already has a determined value {}", self.attempted_val, self.target, solved)?;
            Ok(())
        } else {
            write!(f, "Attempt to write {} to cell {}, but that value was already excluded", self.attempted_val, self.target)?;
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnsolvableError {}

impl Error for UnsolvableError {}

impl fmt::Display for UnsolvableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sudoku cannot be solved")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPuzzleError {}

impl Error for InvalidPuzzleError {}

impl fmt::Display for InvalidPuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given string could not be read as a valid puzzle")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NonUniqueError {}

impl Error for NonUniqueError {}

impl fmt::Display for NonUniqueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sudoku has multiple possible solutions")?;
        Ok(())
    }
}
