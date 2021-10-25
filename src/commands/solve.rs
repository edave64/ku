use std::error::Error;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::errors::{InvalidPuzzleError, UnsolvableError};
use crate::solver::board::Board;
use crate::solver::solve::solve;

pub fn register_command<'a> (app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("solve")
        .about("Solves a given puzzle")
        .arg(Arg::with_name("puzzles")
            .required(true)
            .multiple(true)
            .help("ASD"))
        .arg(Arg::with_name("unambiguous")
            .short("u")
            .long("unambiguous")
            .help("Test the puzzle for ambiguity"))
        .arg(Arg::with_name("pretty")
            .short("p")
            .long("pretty-print")
            .help("Displays the solved puzzles nicely")))
}

pub fn execute (matches: &ArgMatches) -> Result<(),Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("solve") {
        let ambiguity = matches.is_present("unambiguous");
        if let Some(puzzles) = matches.values_of("puzzles") {
            for puzzle in puzzles {
                let board = parse_puzzle(puzzle)?;
                let ret = solve(board, ambiguity)?;
                if let Some(board) = ret {
                    if matches.is_present("pretty") {
                        println!("{}", board);
                    } else {
                        println!("{}", board.to_1d_string());
                    }
                } else {
                    return Err(Box::new(UnsolvableError{}));
                }
            }
        }
    }
    Ok(())
}

fn parse_puzzle (puzzle: &str) -> Result<Board, Box<dyn Error>> {
    let ret: Vec<u8> = puzzle.chars().map(| x | x.to_digit(10).map(|x| x as u8).unwrap_or(0)).collect();
    if ret.len() != 81 { return Err(Box::new(InvalidPuzzleError {})) }
    let board = Board::from_puzzle(ret)?;
    Ok(board)
}
