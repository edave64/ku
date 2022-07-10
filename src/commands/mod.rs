use std::error::Error;
use crate::codex::simple::decode;
use crate::errors::InvalidPuzzleError;
use crate::solver::board::Board;

pub mod solve;
pub mod encode;
pub mod decode;

pub fn parse_puzzle (puzzle: &str) -> Result<Board, Box<dyn Error>> {
    let decoded_puzzle = if puzzle.len() < 81 {
        decode(base64::decode_config(puzzle, base64::URL_SAFE_NO_PAD)?)
    } else {
        puzzle.to_string()
    };

    if decoded_puzzle.len() == 81 {
        let ret: Vec<u8> = decoded_puzzle.chars().map(| x | x.to_digit(10).map(|x| x as u8).unwrap_or(0)).collect();
        let board = Board::from_puzzle(ret)?;
        return Ok(board)
    }
    Err(Box::new(InvalidPuzzleError {}))
}
