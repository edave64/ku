use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::errors::{InvalidPuzzleError, UnsolvableError};
use crate::solver::board::Board;
use crate::solver::solve::solve;
use crate::codex::{predictive,simple};


pub fn register_command<'a> (app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("encode")
        .about("Encodes a puzzle in a smaller format for easier exchange")
        /*.arg(Arg::with_name("puzzles")
            .required(true)
            .multiple(true)
            .help("ASD"))*/
        .arg(Arg::with_name("unambiguous")
            .short("u")
            .long("unambiguous")
            .help("Test the puzzle for ambiguity"))
        .arg(Arg::with_name("pretty")
            .short("p")
            .long("pretty-print")
            .help("Displays the solved puzzles nicely")))
}
/*
pub fn execute (matches: &ArgMatches) -> Result<(),Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("encode") {
        let lines = read_lines("/home/edave/CLionProjects/ku/src/sudoku.csv")?;

        let mut unsolved_compressed: Vec<usize> = vec![];
        let mut solved_compressed: Vec<usize> = vec![];

        for line in lines.skip(1) {
            if line.is_err() { continue; }
            if let Some((unsolved, solved)) = line?.split_once(",") {
                let encoded = simple::encode(unsolved);
                unsolved_compressed.push(encoded.len());
                let decoded = simple::decode(encoded);
                if decoded != unsolved {
                    panic!("Nooooo!!!!")
                }

                let encoded = simple::encode(solved);
                solved_compressed.push(encoded.len());
                let decoded = simple::decode(encoded);
                if decoded != solved {
                    panic!("Nooooo!!!!")
                }
            }
        }

        print_stats("Solved", solved_compressed);
        print_stats("Unsolved", unsolved_compressed);
    }
    Ok(())
}
*/
pub fn execute (matches: &ArgMatches) -> Result<(),Box<dyn Error>> {
    if let Some(matches) = matches.subcommand_matches("encode") {
        let lines = read_lines("/home/edave/CLionProjects/ku/src/sudoku.csv")?;

        let mut hole_occurrences: HashMap<usize, u64> = HashMap::new();
        //let mut chain_occurrences: HashMap<usize, u64> = HashMap::new();

        for line in lines.skip(1) {
            if line.is_err() { continue; }
            if let Some((unsolved, solved)) = line?.split_once(",") {
                let mut hole_size = 0;
                let mut chain_size = 0;
                for char in unsolved.chars() {
                    if char == '0' {
                        if chain_size > 0 {
                            //*chain_occurrences.entry(chain_size).or_insert(0) += 1;
                            chain_size = 0;
                        }
                        hole_size += 1;
                    } else {
                        if chain_size != 0 {
                            *hole_occurrences.entry(0).or_insert(0) += 1;
                        }
                        if hole_size > 0 {
                            *hole_occurrences.entry(hole_size).or_insert(0) += 1;
                            hole_size = 0;
                        }
                        chain_size += 1;
                    }
                }
                if hole_size > 0 {
                    *hole_occurrences.entry(hole_size).or_insert(0) += 1;
                    hole_size = 0;
                }
            }
        }

        let mut keys: Vec<usize> = hole_occurrences.keys().map(|&x| x).collect();
        keys.sort();

        let mut total = 0;
        for key in keys.clone() {
            total += hole_occurrences[&key];
        }


        println!("Holes");
        for key in keys {
            println!("[{}]: {}", key, hole_occurrences[&key] as f64 / total as f64);
        }
        println!();
/*
        let mut keys: Vec<usize> = chain_occurrences.keys().map(|&x| x).collect();
        keys.sort();
        println!("Chain");
        for key in keys {
            println!("[{}]: {}", key, chain_occurrences[&key]);
        }*/
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

fn average(numbers: &[usize]) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn median(numbers: &[usize]) -> i32 {
    let mut clone = numbers.to_owned();
    clone.sort_unstable();
    clone[clone.len() / 2] as i32
}

fn mode(numbers: &[usize]) -> i32 {
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
