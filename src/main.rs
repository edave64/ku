use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use clap::App;

mod codex;
mod solver;
mod errors;
mod commands;

fn main () -> Result<(), Box<dyn Error>> {
    let mut app = App::new("ku")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("A sudoku game/toolkit");

    app = commands::solve::register_command(app);

    let matches = app.get_matches();

    commands::solve::execute(&matches)?;
    Ok(())
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
    let mid = numbers.len() / 2;
    clone[mid] as i32
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
