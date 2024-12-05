const YEAR: u16 = 2024;
const DAY: u8 = 4;
use aoc::vec2d::*;
use aoc::*;
use strum::IntoEnumIterator;

trait Xmas {
    fn get_word(&self, position: Position, direction: Direction) -> Option<[char; 4]>;
    fn get_x(&self, position: Position) -> Option<([char; 3], [char; 3])>;
}

impl Xmas for Vec2d<char> {
    fn get_word(&self, position: Position, direction: Direction) -> Option<[char; 4]> {
        let mut word = [char::default(); 4];

        let dist = Distance::from(direction);

        for (i, c) in word.iter_mut().enumerate() {
            *c = *self.get(position + dist * i)?;
        }

        Some(word)
    }

    fn get_x(&self, position: Position) -> Option<([char; 3], [char; 3])> {
        let mut w1 = [char::default(); 3];
        let mut w2 = [char::default(); 3];

        w1[0] = *self.get(position + Direction::NorthWest.into())?;
        w1[1] = *self.get(position)?;
        w1[2] = *self.get(position + Direction::SouthEast.into())?;

        w2[0] = *self.get(position + Direction::NorthEast.into())?;
        w2[1] = w1[1];
        w2[2] = *self.get(position + Direction::SouthWest.into())?;

        Some((w1, w2))
    }
}

type ResultType = usize;
type DataType = Vec2d<char>;

fn parse(input: &str) -> Result<DataType> {
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    Ok(Vec2d::new(data)?)
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut result = 0;

    for pos in data.size().iter() {
        if data.get(pos).is_some_and(|&c| c == 'X') {
            result += Direction::iter()
                .filter(|&dir| matches!(data.get_word(pos, dir), Some(['X', 'M', 'A', 'S'])))
                .count();
        }
    }

    Ok(result)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut result = 0;

    for pos in data.size().iter() {
        if data.get(pos).is_some_and(|&c| c == 'A') {
            result += match data.get_x(pos) {
                Some((['M', 'A', 'S'], ['M', 'A', 'S'])) => 1,
                Some((['M', 'A', 'S'], ['S', 'A', 'M'])) => 1,
                Some((['S', 'A', 'M'], ['M', 'A', 'S'])) => 1,
                Some((['S', 'A', 'M'], ['S', 'A', 'M'])) => 1,
                _ => 0,
            }
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(18));
        assert_eq!(task2, Some(9));
    }
}
