use crate::errors::{NonUniqueError};
use crate::solver::board::Board;
use crate::solver::board::CellState::{Solved, Unsolved};
use crate::solver::calc::number_to_mask;

pub fn solve(board: Board, prove_unique: bool) -> Result<Option<Board>, NonUniqueError> {
    let next = board.most_certain();
    match next {
        None => return Ok(Some(board)),
        Some((next_cell, possibilities)) => {
            let mut already_found = None;
            for i in 0..9 {
                if possibilities.mask & number_to_mask(i) == 0 { continue; }
                let mut new_board = board.clone();
                let marking = new_board.mark(next_cell, i);
                if marking.is_err() { continue; }

                let sub_ret = solve(new_board, prove_unique);

                if let Ok(Some(solved_board)) = sub_ret {
                    if !prove_unique {
                        return Ok(Some(solved_board));
                    } else if already_found == None {
                        already_found = Some(solved_board);
                    } else {
                        return Err(NonUniqueError {});
                    }
                } else if let Ok(None) = sub_ret {
                    continue;
                } else if let Err(err) = sub_ret {
                    return Err(err);
                }
            }
            if let Some(board) = already_found {
                return Ok(Some(board));
            }
        }
    }

    Ok(None)
}

