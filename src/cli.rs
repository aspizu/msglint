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
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    /// Checks the contents of the given file for problems, If not provided, reads from standard input.
    #[arg()]
    pub file: Option<PathBuf>,
    /// Install hook into `.git/hooks/commit-msg` in the current repository. If provided, all other options are ignored.
    #[arg(short, long)]
    pub install: bool,
}

fn display_problem(problem: &Problem) {
    println!(" {} {}\n", "(!)".bold(), problem.message.yellow())
}

fn read_message_content(cli: Cli) -> anyhow::Result<String> {
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
            "file `.git/hooks/commit-msg` already exists. If it's a shell script, append:\nmsglint \"$1\"\nexit"
        );
    }
    let mut file = File::create(path)?;
    file.write_all("#!/bin/bash\nmsglint \"$1\"\nexit\n".as_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = file.metadata()?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
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
