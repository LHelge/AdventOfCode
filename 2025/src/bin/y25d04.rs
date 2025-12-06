const YEAR: u16 = 2025;
const DAY: u8 = 4;
use aoc::{utils::*, *};
use strum::IntoEnumIterator;

type ResultType = usize;
type DataType = Vec2d<bool>;

fn parse(input: &str) -> Result<DataType> {
    let rolls = input
        .lines()
        .map(|line| line.chars().map(|roll| roll == '@').collect())
        .collect();

    Ok(Vec2d::new(rolls)?)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .filter(|(pos, roll)| {
            if !**roll {
                return false;
            }

            Direction::iter()
                .filter(|&dir| {
                    let pos = *pos + dir.into();
                    data.get(pos).is_some_and(|roll| *roll)
                })
                .count()
                < 4
        })
        .count())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut warehouse = data.clone();
    let mut removed = 0;

    loop {
        let removable: Vec<Position> = warehouse
            .iter()
            .filter_map(|(pos, roll)| {
                if !*roll {
                    return None;
                }

                if Direction::iter()
                    .filter(|&dir| {
                        let pos = pos + dir.into();
                        warehouse.get(pos).is_some_and(|roll| *roll)
                    })
                    .count()
                    < 4
                {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        if removable.is_empty() {
            break;
        }
        removed += removable.len();

        for pos in removable {
            _ = warehouse.set(pos, false);
        }
    }

    Ok(removed)
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
    fn examples() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(13));
        assert_eq!(task2, Some(43));
    }
}
