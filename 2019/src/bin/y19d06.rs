const YEAR: u16 = 2019;
const DAY: u8 = 6;
use std::collections::HashMap;

use aoc::*;

type ResultType = usize;
type DataType = HashMap<String, String>;

fn splitter(row: &str) -> Result<(String, String)> {
    let (object, orbiter) = row.split_once(')').ok_or(AoCError::BadInput)?;

    Ok((orbiter.to_owned(), object.to_owned()))
}

fn parse(input: &str) -> Result<DataType> {
    input.lines().map(splitter).collect()
}

fn count_orbits(orbits: &HashMap<String, String>, object: &str) -> usize {
    match orbits.get(object) {
        Some(object) => count_orbits(orbits, object) + 1,
        None => 0,
    }
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.keys().map(|object| count_orbits(data, object)).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut current = data.get("YOU");
    let mut target = data.get("SAN");

    let mut you_hops: HashMap<String, usize> = HashMap::new();
    let mut santa_hops: HashMap<String, usize> = HashMap::new();

    let mut hops = 0;
    loop {
        if let Some(c) = current {
            you_hops.insert(c.clone(), hops);

            if let Some(san) = santa_hops.get(c) {
                return Ok(san + hops);
            }

            current = data.get(c);
        }
        if let Some(t) = target {
            santa_hops.insert(t.clone(), hops);

            if let Some(you) = you_hops.get(t) {
                return Ok(you + hops);
            }

            target = data.get(t);
        }

        hops += 1;
    }
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(54));
        assert_eq!(task2, Some(4));
    }
}
