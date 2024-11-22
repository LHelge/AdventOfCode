use crate::error::*;
use std::env;

pub struct AoCInput {
    session: String,
}

impl AoCInput {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            session: env::var("SESSION").expect("No SESSION environment variable set"),
        }
    }

    pub fn get_input(&self, year: u16, day: u8) -> Result<String> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let resp = reqwest::blocking::Client::new()
            .get(url)
            .header("Cookie", format!("session={}", self.session))
            .send()?;

        Ok(resp.text()?)
    }
}
