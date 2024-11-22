use std::{error::Error, fmt::Display, num::ParseIntError};

pub type Result<T> = std::result::Result<T, AoCError>;

#[derive(Debug)]
pub enum AoCError {
    Unsolved,
    BadInput,
    BadCharacter(char),
    ParseIntError(ParseIntError),
    ReqwestError(reqwest::Error),
}

impl Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCError::Unsolved => write!(f, "Task has not been solved yet"),
            AoCError::BadInput => write!(f, "Bad input"),
            AoCError::BadCharacter(c) => write!(f, "Bad character: {}", c),
            AoCError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            AoCError::ReqwestError(e) => write!(f, "ReqwestError: {}", e),
        }
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
        AoCError::ReqwestError(e)
    }
}
