use std::{
    fs::{
        self,
        File,
    },
    io::{
        Read,
        Write,
        stdin,
    },
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::bail;
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
    /// Checks the contents of the given file for problems.
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Checks the given message argument for problems. If provided `--file` is ignored.
    #[arg(short, long)]
    pub message: Option<String>,
    /// Install hook into `.git/hooks/commit-msg` in the current repository. If provided, all other options are ignored.
    #[arg(short, long)]
    pub install: bool,
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

fn install() -> anyhow::Result<()> {
    let path = Path::new("./.git/hooks/commit-msg");
    if path.exists() {
        bail!(
            "file `.git/hooks/commit-msg` already exists. If it's a shell script, append:\nmsglint -f \"$1\"\nexit"
        );
    }
    let mut file = File::create(path)?;
    file.write_all("#!/bin/bash\nmsglint -f \"$1\"\nexit\n".as_bytes())?;
    Ok(())
}

pub fn cli() -> anyhow::Result<bool> {
    let cli = Cli::parse();
    if cli.install {
        install()?;
        return Ok(true);
    }
    let message_content = read_message_content(cli)?;
    let mut problems = Problems::new();
    let message = parse_message(&message_content, &mut problems);
    check_all_rules(&message, &mut problems);
    display_problems(&problems);
    Ok(problems.problems.is_empty())
}
