use anyhow::{Context, Result};
use clap::Parser;
use log::{LevelFilter, error, info};
use std::{
    env,
    io::{Write, stdin, stdout},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::cargo_cleaner::CargoCleaner;

pub mod cargo_cleaner;

#[derive(Parser)]
struct UserArgs {
    /// directory
    #[arg(long, short, default_value_os_t = default_directory())]
    directory: PathBuf,

    /// Don't ask user
    #[arg(long, short)]
    yes: bool,

    /// verbose
    #[arg(long, short)]
    verbose: bool,

    /// dry run
    #[arg(long)]
    dry_run: bool,
}

fn default_directory() -> PathBuf {
    env::current_dir().unwrap()
}

fn ask_user(question: &str) -> Result<bool> {
    print!("{question}");

    let mut input = String::new();
    stdout().flush().context("Unable to flush stdout")?;
    stdin()
        .read_line(&mut input)
        .context("Unable to read from stdin")?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(true);
    }

    Ok(input.to_lowercase() == "y")
}

fn clean(cleaner: CargoCleaner, directory: &Path) -> Result<()> {
    println!("Cleaning {}", directory.display());

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() != "target" {
            // not a rust target directory
            continue;
        }

        if !entry.path().is_dir() {
            // not a directory
            continue;
        }

        let Some(cargo_dir) = entry.path().parent() else {
            continue;
        };

        let cargo_toml = cargo_dir.join("Cargo.toml");

        if !cargo_toml.exists() {
            // not a rust directory
            continue;
        }

        info!("{}", cargo_dir.display());

        if let Err(e) = cleaner.clean(cargo_dir) {
            error!("{e}");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = UserArgs::parse();

    let level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    env_logger::Builder::new().filter_level(level).init();

    if !args.yes {
        let q = format!("Cleaning {} [Y/n]: ", args.directory.display());
        if !ask_user(&q)? {
            return Ok(());
        }
    }

    let cleaner = CargoCleaner::new(args.dry_run).context("Unable to initialize cargo cleaner")?;

    clean(cleaner, &args.directory)
}
