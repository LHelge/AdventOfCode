use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    BadYear,
    BadDay,
    TemplateNotFound,
    AlreadyExists,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadYear => write!(f, "Invalid year, should be between 2015 and 2024"),
            Error::BadDay => write!(f, "Invalid day, should be between 1 and 25"),
            Error::TemplateNotFound => write!(f, "Template not found"),
            Error::AlreadyExists => write!(f, "Day already exists"),
            Error::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
