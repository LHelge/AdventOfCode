use std::collections::HashMap;

use aoc::AoCInput;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        match value {
            "byr" => Field::BirthYear,
            "iyr" => Field::IssueYear,
            "eyr" => Field::ExpirationYear,
            "hgt" => Field::Height,
            "hcl" => Field::HairColor,
            "ecl" => Field::EyeColor,
            "pid" => Field::PassportId,
            "cid" => Field::CountryId,
            _ => panic!("Bad input!"),
        }
    }
}

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<Field, &'a str>,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(value: &'a str) -> Self {
        let fields = value.split_whitespace().map(|p| {
            let (key, value) = p.trim().split_once(':').unwrap();

            (key.into(), value)
        }).collect();

        Self { fields }
    }
}

impl Passport<'_> {
    fn is_valid(&self) -> bool {
        self.fields.len() == 8 || (self.fields.len() == 7 && !self.fields.contains_key(&Field::CountryId))
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let passports: Vec<Passport> = input.split("\n\n").map(|p| p.trim().into()).collect();

    let task1 = passports.iter().filter(|p| p.is_valid()).count();
    let task2 = 0;

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2020, 4)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d04 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let (example1, _example2) = solve_task(input);
        assert_eq!(example1, 2);
        //assert_eq!(example2, 0);
    }
}
