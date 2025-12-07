use aoc::{problem::*, utils::*, *};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Entry {
    Start,
    #[default]
    EmptySpace,
    Manifold,
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
        let (start, _) = self.diagram.find(&Entry::Start).ok_or(AoCError::BadInput)?;

        let mut beams: HashSet<Position> = HashSet::new();
        beams.insert(start + Direction::South);

        count_splits(&self.diagram, beams)
    }

    fn part2(&self) -> Result<usize> {
        let (start, _) = self.diagram.find(&Entry::Start).ok_or(AoCError::BadInput)?;
        let beam = start + Direction::South;

        let mut cache: HashMap<Position, usize> = HashMap::new();

        count_timelines(&self.diagram, beam, &mut cache)
    }
}

fn count_splits(diagram: &Vec2d<Entry>, beams: HashSet<Position>) -> Result<usize> {
    let mut new_beams = HashSet::new();
    let mut splits = 0;

    for beam in beams {
        let below = beam + Direction::South;
        match diagram.get(below) {
            Some(Entry::EmptySpace) => new_beams.insert(below),
            Some(Entry::Manifold) => {
                splits += 1;
                new_beams.insert(below + Direction::East);
                new_beams.insert(below + Direction::West)
            }
            None => return Ok(0),
            _ => Err(AoCError::BadInput)?,
        };
    }

    Ok(count_splits(diagram, new_beams)? + splits)
}

fn count_timelines(
    diagram: &Vec2d<Entry>,
    pos: Position,
    cache: &mut HashMap<Position, usize>,
) -> Result<usize> {
    if let Some(timelines) = cache.get(&pos) {
        return Ok(*timelines);
    }

    let below = pos + Direction::South;

    let timelines = match diagram.get(below) {
        Some(Entry::EmptySpace) => count_timelines(diagram, below, cache)?,
        Some(Entry::Manifold) => {
            count_timelines(diagram, below + Direction::East, cache)?
                + count_timelines(diagram, below + Direction::West, cache)?
        }
        None => 1,
        _ => Err(AoCError::BadInput)?,
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
