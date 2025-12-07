use aoc::{problem::*, utils::*, *};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Entry {
    Start,
    #[default]
    EmptySpace,
    Manifold,
    Beam,
}

impl TryFrom<char> for Entry {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'S' => Ok(Entry::Start),
            '.' => Ok(Entry::EmptySpace),
            '^' => Ok(Entry::Manifold),
            c => Err(AoCError::UnexpectedCharacter(c)),
        }
    }
}

#[derive(Default)]
struct Problem {
    diagram: Vec2d<Entry>,
}

impl AoCProblem<usize, usize> for Problem {
    fn date() -> Date {
        Date::new(2025, 7).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.diagram = input.parse()?;
        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        let mut diagram = self.diagram.clone();
        let mut splits = 0;

        let (start, _) = diagram.find(&Entry::Start).ok_or(AoCError::BadInput)?;
        diagram.set(start + Direction::South.into(), Entry::Beam)?;

        for row in (start.y + 1)..diagram.height() {
            let mut beams: HashSet<Position> = HashSet::new();

            for (pos, entry) in diagram.iter_row(row) {
                let above = pos + Direction::North.into();

                if diagram.get(above).is_some_and(|e| *e == Entry::Beam) {
                    match entry {
                        Entry::EmptySpace => beams.insert(pos),
                        Entry::Manifold => {
                            splits += 1;
                            beams.insert(pos + Direction::East.into());
                            beams.insert(pos + Direction::West.into())
                        }
                        _ => Err(AoCError::BadInput)?,
                    };
                }
            }

            for pos in beams {
                diagram.set(pos, Entry::Beam)?;
            }
        }

        Ok(splits)
    }

    fn part2(&self) -> Result<usize> {
        let (beam, _) = self.diagram.find(&Entry::Start).ok_or(AoCError::BadInput)?;

        let mut cache: HashMap<Position, usize> = HashMap::new();

        count_timelines(&self.diagram, beam, &mut cache)
    }
}

fn count_timelines(
    diagram: &Vec2d<Entry>,
    pos: Position,
    cache: &mut HashMap<Position, usize>,
) -> Result<usize> {
    if let Some(timelines) = cache.get(&pos) {
        return Ok(*timelines);
    }

    let below = pos + Direction::South.into();

    let timelines = match diagram.get(below) {
        Some(Entry::EmptySpace) => count_timelines(diagram, below, cache)?,
        Some(Entry::Manifold) => {
            count_timelines(diagram, below + Direction::East.into(), cache)?
                + count_timelines(diagram, below + Direction::West.into(), cache)?
        }
        None => 1,
        _ => panic!("ooooh"),
    };

    cache.insert(pos, timelines);

    Ok(timelines)
}

fn main() -> Result<()> {
    let mut problem = Problem::default();
    let solution = problem.solve()?;

    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(21);
        problem.test_part2(40);
    }
}
