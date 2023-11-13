use reqwest::blocking::Client;

pub fn get_input(year: u16, day: u8, session: &str) -> Result<String, reqwest::Error> {
    let resp = Client::new()
        .get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", format!("session={}", session))
        .send()?;

    Ok(resp.text()?)
}
