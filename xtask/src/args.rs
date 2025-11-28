use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
    pub year: u16,
    pub day: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    New,
    Run,
    Test,
}
