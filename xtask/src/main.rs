use args::*;
use clap::Parser;
use error::*;
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

    match args.command {
        Command::New { year, day } => {
            new_day(workspace_root, year, day)?;
        }
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
