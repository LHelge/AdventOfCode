use reqwest::blocking::Client;

mod error;
pub use error::AoCError;

pub fn get_input(year: u16, day: u8, session: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()
        .expect("Failed to fetch input from Advent of Code");

    Ok(resp.text().expect("Could not get response body"))
}
