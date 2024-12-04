const YEAR: u16 = 2024;
const DAY: u8 = 4;
use std::str::FromStr;

use aoc::*;

struct Board {
    board: Vec<Vec<char>>,
    size: (usize, usize),
}

impl FromStr for Board {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let board: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        // check uniform
        if let Some(first) = board.first() {
            let width = board.len();
            let length = first.len();

            if board.iter().all(|l| l.len() == length) {
                return Ok(Self {
                    board,
                    size: (width, length),
                });
            }
        };

        Err(AoCError::BadInput)
    }
}

impl Board {
    fn char_at(&self, x: usize, y: usize) -> Option<&char> {
        if let Some(line) = self.board.get(x) {
            line.get(y)
        } else {
            None
        }
    }

    fn get_word(
        &self,
        length: usize,
        position: (usize, usize),
        direction: &(isize, isize),
    ) -> Option<String> {
        let mut word = String::new();

        for i in 0..length as isize {
            if let Some(&char) = self.char_at(
                position.0.wrapping_add_signed(i * direction.0),
                position.1.wrapping_add_signed(i * direction.1),
            ) {
                word.push(char);
            } else {
                return None;
            }
        }

        Some(word)
    }

    fn get_x(&self, position: (usize, usize)) -> Option<(String, String)> {
        let mut w1 = String::new();
        let mut w2 = String::new();

        w1.push(*self.char_at(position.0 - 1, position.1 - 1)?);
        w1.push(*self.char_at(position.0, position.1)?);
        w1.push(*self.char_at(position.0 + 1, position.1 + 1)?);

        w2.push(*self.char_at(position.0 - 1, position.1 + 1)?);
        w2.push(*self.char_at(position.0, position.1)?);
        w2.push(*self.char_at(position.0 + 1, position.1 - 1)?);

        Some((w1, w2))
    }
}

type ResultType = usize;
type DataType = Board;

fn parse(input: &str) -> Result<DataType> {
    input.parse()
}

fn task1(data: &DataType) -> Result<ResultType> {
    let directions: [(isize, isize); 8] = [
        (1, 0),   // South
        (1, 1),   // South east
        (0, 1),   // East
        (-1, 1),  // North east
        (-1, 0),  // North
        (-1, -1), // North west
        (0, -1),  // west
        (1, -1),  // South west
    ];

    let mut task1 = 0;

    for x in 0..data.size.0 {
        for y in 0..data.size.1 {
            if matches!(data.char_at(x, y), Some('X')) {
                task1 += directions
                    .iter()
                    .filter(
                        |&dir| matches!(data.get_word(4, (x,y), dir), Some(word) if word == "XMAS"),
                    )
                    .count();
            }
        }
    }

    Ok(task1)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut task2 = 0;

    for x in 1..(data.size.0 - 1) {
        for y in 1..(data.size.1 - 1) {
            if matches!(data.char_at(x, y), Some('A')) {
                match data.get_x((x, y)) {
                    Some((w1, w2)) if w1 == "MAS" && w2 == "MAS" => task2 += 1,
                    Some((w1, w2)) if w1 == "MAS" && w2 == "SAM" => task2 += 1,
                    Some((w1, w2)) if w1 == "SAM" && w2 == "MAS" => task2 += 1,
                    Some((w1, w2)) if w1 == "SAM" && w2 == "SAM" => task2 += 1,
                    _ => (),
                }
            }
        }
    }

    Ok(task2)
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod y2024d02 {
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
