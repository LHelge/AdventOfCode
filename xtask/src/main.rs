use args::*;
use clap::Parser;
use error::*;
use std::env;
use std::io::Write;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

mod args;
mod error;

fn main() -> Result<()> {
    let args = Args::parse();

    let xtask_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = xtask_root.ancestors().nth(1).unwrap();

    let (year, day) = match (args.year, args.day) {
        (y @ 2025, d @ 1..=12) => Ok((y, d)),
        (y @ 2015..=2024, d @ 1..=25) => Ok((y, d)),
        (y, d) => Err(Error::InvalidDay(y, d)),
    }?;

    match args.command {
        None | Some(Command::Run) => run_day(workspace_root, year, day)?,
        Some(Command::Test) => test_day(workspace_root, year, day)?,
        Some(Command::New) => new_day(workspace_root, year, day)?,
    }

    Ok(())
}

fn run_day(workspace_root: &Path, year: u16, day: u8) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let year = year % 100;
    let binary = format!("y{year:02}d{day:02}");

    let cmd = std::process::Command::new(cargo)
        .current_dir(workspace_root)
        .args(["run", "--release", "--bin", &binary])
        .status()?;

    if !cmd.success() {
        Err(Error::RunFailed(year, day))?;
    }
    Ok(())
}

fn test_day(workspace_root: &Path, year: u16, day: u8) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let year = year % 100;
    let binary = format!("y{year:02}d{day:02}");

    let cmd = std::process::Command::new(cargo)
        .current_dir(workspace_root)
        .args(["test", "--bin", &binary])
        .status()?;

    if !cmd.success() {
        Err(Error::TestsFailed(year, day))?;
    }
    Ok(())
}

fn new_day(workspace_root: &Path, year: u16, day: u8) -> Result<()> {
    match (year, day) {
        (2025, 1..=12) => Ok(()),
        (2015..=2024, 1..=25) => Ok(()),
        (year, day) => Err(Error::InvalidDay(year, day)),
    }?;

    let template = workspace_root.join("day_template.rs");
    if !template.exists() {
        return Err(Error::TemplateNotFound);
    }

    let target = workspace_root
        .join(year.to_string())
        .join("src")
        .join("bin")
        .join(format!("y{}d{:02}.rs", year - 2000, day));
    if target.exists() {
        return Err(Error::AlreadyExists(year, day));
    }

    let template = fs::read_to_string(template)?;
    let mut output = File::create_new(target)?;
    writeln!(output, "const YEAR: u16 = {year};")?;
    writeln!(output, "const DAY: u8 = {day};")?;
    output.write_all(template.as_bytes())?;

    Ok(())
}
