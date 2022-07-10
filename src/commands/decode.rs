use std::error::Error;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::commands::parse_puzzle;

pub fn register_command<'a> (app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("decode")
        .about("Decodes puzzles to the standard 81 number notation")
        .arg(Arg::with_name("puzzles")
            .required(true)
            .multiple(true)
            .help("The puzzles to solve"))
        .arg(Arg::with_name("pretty")
            .short("p")
            .long("pretty-print")
            .help("Displays the decoded puzzles nicely")))
}
pub fn execute (matches: &ArgMatches) -> Result<(),Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("decode") {
        if let Some(puzzles) = matches.values_of("puzzles") {
            for puzzle in puzzles {
                let board = parse_puzzle(puzzle)?;
                if matches.is_present("pretty") {
                    println!("{}", board);
                } else {
                    println!("{}", board.to_1d_string());
                }
            }
        }
    }
    Ok(())
}
