use clap::App;
use std::error::Error;

mod codex;
mod commands;
mod errors;
mod solver;
mod tools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new("ku")
        .version("0.2")
        .author("edave64 <edave64@gmail.com>")
        .about("A sudoku game/toolkit");

    app = commands::solve::register_command(app);
    app = commands::encode::register_command(app);
    app = commands::decode::register_command(app);

    let matches = app.get_matches();

    commands::solve::execute(&matches)?;
    commands::encode::execute(&matches)?;
    commands::decode::execute(&matches)?;
    Ok(())
}
