use crate::error::*;
use reqwest::blocking::Client;
use std::env;

pub struct AoCInput {
    session: String,
}

impl AoCInput {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            session: env::var("SESSION")?,
        })
    }

    pub fn get_input(&self, year: u16, day: u8) -> Result<String> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let resp = Client::new()
            .get(url)
            .header("Cookie", format!("session={}", self.session))
            .send()?;

        if !resp.status().is_success() {
            Err(AoCError::FetchInput(resp.text()?))
        } else {
            Ok(resp.text()?.trim().to_string())
        }
    }
}
