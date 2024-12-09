const YEAR: u16 = 2024;
const DAY: u8 = 9;
use aoc::*;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    start: usize,
    size: usize,
    space: usize,
}

impl File {
    fn end(&self) -> usize {
        self.start + self.size
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;
        for i in self.start..(self.start + self.size) {
            checksum += i * self.id;
        }
        checksum
    }
}

type ResultType = usize;
type DataType = Vec<File>;

fn parse(input: &str) -> Result<DataType> {
    let mut files = vec![];

    let mut chars = input.chars();
    let mut id = 0;
    let mut start = 0;
    while let (Some(size), space) = (chars.next(), chars.next()) {
        let size = size.to_digit(10).ok_or(AoCError::BadInput)? as usize;
        let space = space
            .unwrap_or('0')
            .to_digit(10)
            .ok_or(AoCError::BadInput)? as usize;

        files.push(File {
            id,
            start,
            size,
            space,
        });

        id += 1;
        start += size + space;
    }

    Ok(files)
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut disk: Vec<Option<usize>> = vec![];

    for file in data {
        for _ in 0..file.size {
            disk.push(Some(file.id));
        }
        for _ in 0..file.space {
            disk.push(None);
        }
    }

    while disk.iter().any(|f| f.is_none()) {
        if let Some(last) = disk.pop() {
            if let Some(first_empty_index) = disk.iter().position(|f| f.is_none()) {
                disk.remove(first_empty_index);
                disk.insert(first_empty_index, last);
            }
        }
    }

    Ok(disk.iter().enumerate().map(|(i, f)| i * f.unwrap()).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut disk = data.clone();

    for id in (1..data.len()).rev() {
        if let Some(first_free_id) = disk
            .iter()
            .position(|f| f.space >= disk[id].size && f.start < disk[id].start)
        {
            // Give free space to the file before
            disk[id - 1].space += disk[id].size + disk[id].space;

            // Move the file to the first free space
            disk[id].start = disk[first_free_id].end();
            disk[id].space = disk[first_free_id].space - disk[id].size;

            // Remove the free space
            disk[first_free_id].space = 0;
        }

        disk.sort_by(|a, b| a.start.cmp(&b.start));
    }

    Ok(disk.iter().map(File::checksum).sum())
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
        let input = "2333133121414131402";

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1928));
        assert_eq!(task2, Some(2858));
    }
}
