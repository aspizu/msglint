use std::{
    process::ExitCode,
    time::Instant,
};

use colored::{
    Color,
    Colorize,
};

use crate::cli::cli;

mod cli;
mod messages;
mod problems;
mod rules;

fn main() -> ExitCode {
    let begin = Instant::now();
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "{info}\n{}\nopen an issue at {}",
            "msglint is cooked 💀".red().bold(),
            "https://github.com/aspizu/msglint/issues".cyan()
        );
    }));
    let result = cli();
    if let Err(error) = &result {
        eprintln!("{}{} {error}", "error".bold().red(), ":".bold());
    }
    let color = if result.is_ok() {
        Color::Green
    } else {
        Color::Red
    };
    eprintln!(
        "{} in {:?}",
        "Finished".color(color).bold(),
        begin.elapsed()
    );
    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
