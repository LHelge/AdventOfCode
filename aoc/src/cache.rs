use reqwest::blocking::Client;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use thiserror::Error;

use crate::problem::Date;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Problem fetching input: {0}")]
    FetchInput(String),
}

type Result<T> = std::result::Result<T, CacheError>;

pub struct Cache {
    dir: PathBuf,
    session: String,
}

impl Cache {
    pub fn new(dir: impl Into<PathBuf>, session: impl Into<String>) -> Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        Ok(Self {
            dir,
            session: session.into(),
        })
    }

    pub fn get(&self, date: Date) -> Result<String> {
        let filename = self.dir.join(date.filename());

        if filename.exists() {
            Ok(fs::read_to_string(filename)?)
        } else {
            let resp = Client::new()
                .get(date.url())
                .header("Cookie", format!("session={}", self.session))
                .send()?;

            if !resp.status().is_success() {
                Err(CacheError::FetchInput(resp.text()?))
            } else {
                let input = resp.text()?.to_string();

                let mut file = File::create(filename)?;
                file.write_all(input.as_bytes())?;
                Ok(input)
            }
        }
    }
}
