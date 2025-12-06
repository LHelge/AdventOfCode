use std::num::ParseIntError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AoCError>;

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("Bad input data from Advent of Code")]
    BadInput,

    #[error("Unexpected character {0} in input data")]
    UnexpectedCharacter(char),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error("Could not fetch input, check your SESSION environment variable: {0}")]
    CouldNotFetchInput(#[from] reqwest::Error), // TODO: Remove this once all problems refactored to trait based solver

    #[error("Task not solved yet")]
    Unsolved,

    #[error("Environment variable error: {0}")]
    EnvironmentVariable(#[from] std::env::VarError),

    #[error("Could not fetch input: {0}")]
    FetchInput(String),

    #[error("Nom parser error")]
    Parser,

    #[error(transparent)]
    Vec2d(#[from] crate::utils::Vec2dError),

    #[error(transparent)]
    IntCode(#[from] crate::intcode::IntCodeError),

    #[error("The date {0} day {1} is not a valid AoC problem")]
    InvalidDay(u16, u8),

    #[error(transparent)]
    Cache(#[from] crate::cache::CacheError),
}

impl<I> From<nom::error::Error<I>> for AoCError {
    fn from(_: nom::error::Error<I>) -> Self {
        AoCError::Parser
    }
}
