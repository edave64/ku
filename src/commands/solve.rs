use std::error::Error;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::commands::parse_puzzle;
use crate::errors::{UnsolvableError};
use crate::solver::solve::solve;

pub fn register_command<'a> (app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("solve")
        .about("Solves a given puzzle")
        .arg(Arg::with_name("puzzles")
            .required(true)
            .multiple(true)
            .help("The puzzles to solve"))
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
