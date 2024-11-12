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

const VALID_EYE_COLORS: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
];

impl Field {
    fn is_valid_value(&self, value: &str) -> bool {
        match self {
            Field::BirthYear => (1920..=2002).contains(&value.parse().unwrap()),
            Field::IssueYear => (2010..=2020).contains(&value.parse().unwrap()),
            Field::ExpirationYear => (2020..=2030).contains(&value.parse().unwrap()),
            Field::Height => Self::is_height_valid(value),
            Field::HairColor => value.strip_prefix('#').is_some_and(|c| c.chars().all(|c| c.is_ascii_hexdigit())),
            Field::EyeColor => VALID_EYE_COLORS.contains(&value),
            Field::PassportId => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
            Field::CountryId => true,
        }
    }

    fn is_height_valid(value: &str) -> bool {
        if let Some(centimeters) = value.strip_suffix("cm") {
            if let Ok(centimeters) = centimeters.parse::<u16>() {
                return (150..=193).contains(&centimeters);
            };
        };
        
        if let Some(inches) = value.strip_suffix("in") {
            if let Ok(inches) = inches.parse::<u16>() {
                return (59..=76).contains(&inches);
            }
        };

        false
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

const REQUIRED_FIELDS: [Field; 7] = [
    Field::BirthYear,
    Field::IssueYear,
    Field::ExpirationYear,
    Field::Height,
    Field::HairColor,
    Field::EyeColor,
    Field::PassportId,
];

impl Passport<'_> {
    fn is_valid1(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|f| self.fields.contains_key(f))
    }

    fn is_valid2(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|f| {
            self.fields.get(f).is_some_and(|&v| f.is_valid_value(v) )
        })
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let passports: Vec<Passport> = input.split("\n\n").map(|p| p.trim().into()).collect();

    let task1 = passports.iter().filter(|p| p.is_valid1()).count();
    let task2 = passports.iter().filter(|p| p.is_valid2()).count();

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
    fn examples1() {
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
        let (example1, _) = solve_task(input);
        assert_eq!(example1, 2);
    }

    #[test]
    fn examples2() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        let (_, all_invalid) = solve_task(input);
        assert_eq!(all_invalid, 0);

        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let (_, all_valid) = solve_task(input);
        assert_eq!(all_valid, 4);
    }
}
