use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Day {1}, {0} does have an Advent of Code problem")]
    InvalidDay(u16, u8),

    #[error("Template not found")]
    TemplateNotFound,

    #[error("Day {1}, {0} already exists")]
    AlreadyExists(u16, u8),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to run day {1}, {0}")]
    RunFailed(u16, u8),

    #[error("Tests failed for day {1}, {0}")]
    TestsFailed(u16, u8),
}
