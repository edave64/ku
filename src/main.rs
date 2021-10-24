use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use crate::errors::{InvalidPuzzleError, UnsolvableError};
use crate::solver::board::Board;

mod codex;
mod solver;
mod errors;

extern crate clap;
use clap::{Arg, App, SubCommand};
use crate::solver::solve::solve;

fn main () -> Result<(), Box<dyn Error>> {
    let matches = App::new("ku")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("A sudoku game/toolkit")
        .subcommand(SubCommand::with_name("solve")
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
        .get_matches();

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

fn print_stats(headline: &str, collection: Vec<usize>) {
    println!("{}", headline);
    println!("{}", "=".repeat(headline.len()));
    println!("average: {} bytes", average(&collection));
    println!("median: {} bytes", median(&collection));
    println!("mode: {} bytes", mode(&collection));
    println!("max: {} bytes", collection.iter().max().unwrap());
    println!("min: {} bytes", collection.iter().min().unwrap());
}

fn average(numbers: &Vec<usize>) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<usize>) -> i32 {
    let mut clone = numbers.to_owned();
    clone.sort();
    let mid = numbers.len() / 2;
    clone[mid] as i32
}

fn mode(numbers: &Vec<usize>) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers") as i32
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
