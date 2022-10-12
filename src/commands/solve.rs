use crate::commands::parse_puzzle;
use crate::errors::UnsolvableError;
use crate::solver::solve::solve;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

const COMMAND_NAME: &str = "solve";
const UNAMBIGUOUS: &str = "unambiguous";
const PUZZLES: &str = "puzzles";
const PRETTY: &str = "pretty";

pub fn register_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(
        SubCommand::with_name(COMMAND_NAME)
            .about("Solves a given puzzle")
            .arg(
                Arg::with_name(PUZZLES)
                    .required(true)
                    .multiple(true)
                    .help("The puzzles to solve"),
            )
            .arg(
                Arg::with_name(UNAMBIGUOUS)
                    .short("u")
                    .long("unambiguous")
                    .help("Test the puzzle for ambiguity"),
            )
            .arg(
                Arg::with_name(PRETTY)
                    .short("p")
                    .long("pretty-print")
                    .help("Displays the solved puzzles nicely"),
            ),
    )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let matches = match matches.subcommand_matches(COMMAND_NAME) {
        Some(matches) => matches,
        _ => return Ok(()),
    };
    let puzzles = match matches.values_of(PUZZLES) {
        Some(puzzles) => puzzles,
        _ => return Ok(()),
    };
    let ambiguity = matches.is_present(UNAMBIGUOUS);
    for puzzle in puzzles {
        let board = parse_puzzle(puzzle)?;
        let ret = solve(board, ambiguity)?;
        if let Some(board) = ret {
            if matches.is_present(PRETTY) {
                println!("{}", board);
            } else {
                println!("{}", board.to_1d_string());
            }
        } else {
            return Err(Box::new(UnsolvableError {}));
        }
    }
    Ok(())
}
