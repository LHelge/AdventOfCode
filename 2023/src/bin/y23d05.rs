use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

use aoc::AoCError;

#[derive(Debug)]
pub struct Map {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl FromStr for Map {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let destination_start = parts.next().ok_or(AoCError::BadInput)?.parse()?;
        let source_start = parts.next().ok_or(AoCError::BadInput)?.parse()?;
        let length = parts.next().ok_or(AoCError::BadInput)?.parse()?;

        Ok(Map {
            source_start,
            destination_start,
            length,
        })
    }
}

impl Map {
    fn map(&self, input: u64) -> Option<u64> {
        if input < self.source_start || input >= self.source_start + self.length {
            None
        } else {
            Some(self.destination_start + input.abs_diff(self.source_start))
        }
    }
}

#[derive(Debug)]
struct MapGroup {
    from: String,
    _to: String,
    maps: Vec<Map>,
}

impl FromStr for MapGroup {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (from, to) = lines
            .next()
            .ok_or(AoCError::BadInput)?
            .split_once(' ')
            .ok_or(AoCError::BadInput)?
            .0
            .split_once("-to-")
            .ok_or(AoCError::BadInput)?;

        let maps = lines.map(|m| m.parse().expect("Bad input")).collect();

        Ok(MapGroup {
            from: String::from(from),
            _to: String::from(to),
            maps,
        })
    }
}

impl MapGroup {
    fn map(&self, input: u64) -> u64 {
        self.maps
            .iter()
            .filter_map(|f| f.map(input))
            .next()
            .unwrap_or(input)
    }
}

fn solve_task(input: &str) -> Result<(u64, u64), AoCError> {
    let mut parts = input.split("\n\n");

    let seeds = parts
        .next()
        .ok_or(AoCError::BadInput)?
        .strip_prefix("seeds: ")
        .ok_or(AoCError::BadInput)?
        .split_whitespace()
        .map(|s| s.parse().expect("Bad input"))
        .collect::<Vec<u64>>();

    let map_groups = parts
        .map(|m| {
            let map_group = m.parse::<MapGroup>().expect("Bad input");
            (map_group.from.clone(), map_group)
        })
        .collect::<HashMap<String, MapGroup>>();

    let mut task1 = u64::MAX;
    for seed in seeds.iter() {
        let soil = map_groups.get("seed").unwrap().map(*seed);
        let fertilizer = map_groups.get("soil").unwrap().map(soil);
        let water = map_groups.get("fertilizer").unwrap().map(fertilizer);
        let light = map_groups.get("water").unwrap().map(water);
        let temperature = map_groups.get("light").unwrap().map(light);
        let humidity = map_groups.get("temperature").unwrap().map(temperature);
        let location = map_groups.get("humidity").unwrap().map(humidity);

        //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

        task1 = task1.min(location);
    }

    let mut task2 = u64::MAX;
    let mut visited: HashSet<u64> = HashSet::new();
    for seed_range in seeds.chunks(2).map(|c| c[0]..(c[0] + c[1])) {
        println!("\nSeed range: {:?}", seed_range);
        for seed in seed_range {
            if visited.contains(&seed) {
                continue;
            }
            visited.insert(seed);

            let soil = map_groups.get("seed").unwrap().map(seed);
            let fertilizer = map_groups.get("soil").unwrap().map(soil);
            let water = map_groups.get("fertilizer").unwrap().map(fertilizer);
            let light = map_groups.get("water").unwrap().map(water);
            let temperature = map_groups.get("light").unwrap().map(light);
            let humidity = map_groups.get("temperature").unwrap().map(temperature);
            let location = map_groups.get("humidity").unwrap().map(humidity);

            //println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

            task2 = task2.min(location);
            print!("\r{}", seed);
        }
    }

    Ok((task1, task2))
}

fn main() {
    let input = aoc::get_input(
        2023,
        5,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input).expect("Error while solving task");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d04 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let (example1, example2) = solve_task(example_input).expect("Error while solving task");

        assert_eq!(example1, 35);
        assert_eq!(example2, 46);
    }
}
