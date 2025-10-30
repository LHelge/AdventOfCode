use std::{
    error::Error,
    fmt::{Debug, Display},
    num::ParseIntError,
};

use crate::intcode::IntCodeError;

pub type Result<T> = std::result::Result<T, AoCError>;

pub enum AoCError {
    BadInput,
    BadCharacter(char),
    ParseIntError(ParseIntError),
    CouldNotFetchInput(reqwest::Error),
    Unsolved,
    EnvironmentVariable(std::env::VarError),
    FetchInput(String),
    Parser,
    Vec2d(crate::vec2d::Error),
    IntCode(IntCodeError),
}

impl Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCError::BadInput => write!(f, "Bad input"),
            AoCError::BadCharacter(c) => write!(f, "Bad character: {}", c),
            AoCError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            AoCError::CouldNotFetchInput(e) => write!(
                f,
                "Could not fetch input, please check your SESSION environment variable. ({e})"
            ),
            AoCError::Unsolved => write!(f, "Task not solved yet"),
            AoCError::EnvironmentVariable(e) => write!(f, "Environment variable error: {e}"),
            AoCError::FetchInput(msg) => write!(f, "Could not fetch input: {msg}"),
            AoCError::Parser => write!(f, "Parser error"),
            AoCError::Vec2d(e) => write!(f, "Vec2d error: {e}"),
            AoCError::IntCode(e) => write!(f, "Intcode error: {e}"),
        }
    }
}

impl Debug for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for AoCError {}

impl From<ParseIntError> for AoCError {
    fn from(e: std::num::ParseIntError) -> Self {
        AoCError::ParseIntError(e)
    }
}

impl From<reqwest::Error> for AoCError {
    fn from(e: reqwest::Error) -> Self {
        AoCError::CouldNotFetchInput(e)
    }
}

impl From<std::env::VarError> for AoCError {
    fn from(e: std::env::VarError) -> Self {
        AoCError::EnvironmentVariable(e)
    }
}

impl<I> From<nom::error::Error<I>> for AoCError {
    fn from(_: nom::error::Error<I>) -> Self {
        AoCError::Parser
    }
}

impl From<crate::vec2d::Error> for AoCError {
    fn from(e: crate::vec2d::Error) -> Self {
        AoCError::Vec2d(e)
    }
}

impl From<crate::intcode::IntCodeError> for AoCError {
    fn from(e: crate::intcode::IntCodeError) -> Self {
        AoCError::IntCode(e)
    }
}
