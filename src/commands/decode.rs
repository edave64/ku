use crate::commands::parse_puzzle;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

const COMMAND_NAME: &str = "decode";
const PUZZLES: &str = "puzzles";
const PRETTY: &str = "pretty";

pub fn register_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(
        SubCommand::with_name(COMMAND_NAME)
            .about("Decodes puzzles to the standard 81 number notation")
            .arg(
                Arg::with_name(PUZZLES)
                    .required(true)
                    .multiple(true)
                    .help("The puzzles to solve"),
            )
            .arg(
                Arg::with_name(PRETTY)
                    .short("p")
                    .long("pretty-print")
                    .help("Displays the decoded puzzles nicely"),
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
    for puzzle in puzzles {
        let board = parse_puzzle(puzzle)?;
        if matches.is_present(PRETTY) {
            println!("{}", board);
        } else {
            println!("{}", board.to_1d_string());
        }
    }
    Ok(())
}
