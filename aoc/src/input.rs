use std::env;

use reqwest::blocking::Client;

pub struct AoCInput {
    session: String,
}

impl AoCInput {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self { session: env::var("SESSION").expect("No SESSION environment variable set") }
    }

    pub fn get_input(&self, year: u16, day: u8) -> reqwest::Result<String> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let resp = Client::new()
            .get(url)
            .header("Cookie", format!("session={}", self.session))
            .send()
            .expect("Failed to fetch input from Advent of Code");
        
        Ok(resp.text().expect("Could not get response body"))
    }
}
