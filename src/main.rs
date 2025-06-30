use std::process::ExitCode;

use colored::Colorize;
use msglint::cli::cli;

fn main() -> ExitCode {
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
    if result.is_ok_and(|result| result) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
