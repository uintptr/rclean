use crate::cleaner::CargoCleaner;
use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use log::{LevelFilter, error, info};
use rstaples::display::fmt_size;
use std::{
    env,
    io::{Write, stdin, stdout},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub mod cleaner;

#[derive(Parser)]
struct UserArgs {
    /// Don't ask user
    #[arg(long, short)]
    yes: bool,

    /// verbose
    #[arg(long, short)]
    verbose: bool,

    /// dry run
    #[arg(long)]
    dry_run: bool,

    /// directory
    #[arg(default_value_os_t = default_directory().expect("Unable to find default directory"))]
    directory: PathBuf,
}

fn default_directory() -> Result<PathBuf> {
    Ok(env::current_dir()?)
}

fn ask_user(question: &str) -> Result<bool> {
    print!("{question}");

    let mut input = String::new();
    stdout().flush().context("Unable to flush stdout")?;
    stdin().read_line(&mut input).context("Unable to read from stdin")?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(true);
    }

    Ok(input.to_lowercase() == "y")
}

fn dir_size(directory: &Path) -> u64 {
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        let Ok(m) = entry.metadata() else {
            continue;
        };

        total_size = total_size.saturating_add(m.len());
    }
    total_size
}

fn clean(cleaner: &CargoCleaner, directory: &Path) -> Result<()> {
    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
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

        let size = dir_size(cargo_dir);

        print!("Cleaning: {}", cargo_dir.display());
        stdout().flush().context("Unable to flush stdout")?;

        if let Err(e) = cleaner.clean(cargo_dir) {
            error!("{e}");
        }

        println!(" {}", fmt_size(size).green());
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

    let directory = args
        .directory
        .canonicalize()
        .with_context(|| format!("Unable to canonicalize {}", args.directory.display()))?;

    if !args.yes {
        let q = format!("Cleaning {} [Y/n]: ", directory.display());
        if !ask_user(&q)? {
            return Ok(());
        }
    }

    let cleaner = CargoCleaner::new(args.dry_run).context("Unable to initialize cargo cleaner")?;

    clean(&cleaner, &directory)
}
