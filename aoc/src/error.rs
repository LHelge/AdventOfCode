use std::{error::Error, fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum AoCError {
    BadInput,
    BadCharacter(char),
    ParseIntError(ParseIntError),
}

impl Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCError::BadInput => write!(f, "Bad input"),
            AoCError::BadCharacter(c) => write!(f, "Bad character: {}", c),
            AoCError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
        }
    }
}

impl Error for AoCError {}

impl From<ParseIntError> for AoCError {
    fn from(e: std::num::ParseIntError) -> Self {
        AoCError::ParseIntError(e)
    }
}
