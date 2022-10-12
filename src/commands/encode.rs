use crate::codex::simple::encode;
use crate::commands::parse_puzzle;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

const COMMAND_NAME: &str = "encode";
const PUZZLES: &str = "puzzles";

pub fn register_command<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(
        SubCommand::with_name(COMMAND_NAME)
            .about("Encodes a puzzle in a smaller format for easier exchange")
            .arg(
                Arg::with_name(PUZZLES)
                    .required(true)
                    .multiple(true)
                    .help("The puzzles to solve"),
            ),
    )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let matches = match matches.subcommand_matches(COMMAND_NAME) {
        Some(matches) => matches,
        _ => return Ok(()),
    };
    if let Some(puzzles) = matches.values_of(PUZZLES) {
        for puzzle in puzzles {
            let board = parse_puzzle(puzzle)?;
            println!(
                "{}",
                base64::encode_config(&encode(&board.to_1d_string()), base64::URL_SAFE_NO_PAD)
            );
        }
    }
    Ok(())
}
