use std::error::Error;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::codex::simple::{encode};
use crate::commands::parse_puzzle;

pub fn register_command<'a> (app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("encode")
        .about("Encodes a puzzle in a smaller format for easier exchange")
        .arg(Arg::with_name("puzzles")
            .required(true)
            .multiple(true)
            .help("The puzzles to solve")))
}
pub fn execute (matches: &ArgMatches) -> Result<(),Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("encode") {
        if let Some(puzzles) = matches.values_of("puzzles") {
            for puzzle in puzzles {
                let board = parse_puzzle(puzzle)?;
                println!("{}", base64::encode_config(&encode(&board.to_1d_string()), base64::URL_SAFE_NO_PAD));
            }
        }
    }
    Ok(())
}
