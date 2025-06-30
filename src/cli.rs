use std::{
    fs,
    io::{
        Read,
        stdin,
    },
    path::PathBuf,
};

use clap::Parser;
use colored::Colorize;

use crate::{
    messages::parse_message,
    problems::{
        Problem,
        Problems,
    },
    rules::check_all_rules,
};

#[derive(Parser)]
#[command(version=env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    #[arg(short, long)]
    pub message: Option<String>,
}

fn display_problem(problem: &Problem) {
    println!(" {} {}\n", "(!)".bold(), problem.message.yellow())
}

fn read_message_content(cli: Cli) -> anyhow::Result<String> {
    if let Some(message_content) = cli.message {
        return Ok(message_content);
    }
    if let Some(file_path) = cli.file {
        Ok(fs::read_to_string(file_path)?)
    } else {
        let mut s = String::new();
        stdin().read_to_string(&mut s)?;
        Ok(s)
    }
}

fn display_problems(problems: &Problems) {
    for problem in &problems.problems {
        display_problem(problem);
    }
    if problems.problems.len() == 1 {
        println!("{}", "Found 1 problem".red().bold())
    } else if problems.problems.len() > 1 {
        println!(
            "{}{}{}",
            "Found ".red().bold(),
            problems.problems.len().to_string().red().bold(),
            " problems".red().bold()
        )
    }
}

pub fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let message_content = read_message_content(cli)?;
    let mut problems = Problems::new();
    let message = parse_message(&message_content, &mut problems);
    check_all_rules(&message, &mut problems);
    display_problems(&problems);
    Ok(())
}
