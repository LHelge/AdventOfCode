const YEAR: u16 = 2024;
const DAY: u8 = 9;
use std::cmp::Ordering;

use aoc::*;

trait HashChecksum {
    fn checksum(&self) -> usize;
}

impl HashChecksum for Vec<DiskEntity> {
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut block = 0usize;

        for &de in self.iter() {
            match de {
                DiskEntity::File { size, id } => {
                    for _ in 0..size {
                        checksum += block * id;
                        block += 1;
                    }
                }
                DiskEntity::Space { space } => {
                    block += space;
                }
            }
        }

        checksum
    }
}

#[derive(Debug, Clone, Copy)]
enum DiskEntity {
    Space { space: usize },
    File { size: usize, id: usize },
}

impl DiskEntity {
    fn is_space(&self) -> bool {
        matches!(self, DiskEntity::Space { space: _ })
    }
}

type ResultType = usize;
type DataType = Vec<DiskEntity>;

fn parse(input: &str) -> Result<DataType> {
    let mut disk = vec![];

    let mut chars = input.chars();
    let mut id = 0;
    while let (Some(size), space) = (chars.next(), chars.next()) {
        let size = size.to_digit(10).ok_or(AoCError::BadInput)? as usize;
        let space = space
            .unwrap_or('0')
            .to_digit(10)
            .ok_or(AoCError::BadInput)? as usize;

        disk.push(DiskEntity::File { size, id });
        id += 1;

        if space > 0 {
            disk.push(DiskEntity::Space { space });
        }
    }

    Ok(disk)
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut disk = data.clone();

    while disk.iter().any(DiskEntity::is_space) {
        if let Some(DiskEntity::File { size, id }) = disk.pop() {
            let mut size_left = size;

            while size_left > 0 {
                if let Some(index) = disk.iter().position(DiskEntity::is_space) {
                    if let DiskEntity::Space { space } = disk[index] {
                        match space.cmp(&size_left) {
                            Ordering::Less => {
                                // Space is smaller than file, take chunk
                                disk.remove(index);
                                disk.insert(index, DiskEntity::File { size: space, id });
                                size_left -= space;
                            }
                            Ordering::Greater => {
                                // Space is larger than file, replace part of it
                                disk.remove(index);
                                disk.insert(
                                    index,
                                    DiskEntity::File {
                                        size: size_left,
                                        id,
                                    },
                                );
                                disk.insert(
                                    index + 1,
                                    DiskEntity::Space {
                                        space: space - size_left,
                                    },
                                );
                                size_left = 0;
                            }
                            Ordering::Equal => {
                                //space and file are same size, replace it
                                disk.remove(index);
                                disk.insert(
                                    index,
                                    DiskEntity::File {
                                        size: size_left,
                                        id,
                                    },
                                );
                                size_left = 0;
                            }
                        }
                    }
                } else {
                    // No free space left, put last.
                    disk.push(DiskEntity::File {
                        size: size_left,
                        id,
                    });
                    size_left = 0;
                };
            }
        }
    }

    Ok(disk.checksum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut disk = data.clone();

    let mut ids = disk
        .iter()
        .filter_map(|&de| match de {
            DiskEntity::File { size: _, id } => Some(id),
            _ => None,
        })
        .collect::<Vec<usize>>();
    ids.sort();

    for &move_id in ids.iter().rev() {
        if let Some(index) = disk
            .iter()
            .position(|de| matches!(de, DiskEntity::File { size: _, id } if id == &move_id))
        {
            if let DiskEntity::File { size, id } = disk[index] {
                if let Some(free_index) = disk
                    .iter()
                    .position(|de| matches!(de, DiskEntity::Space { space } if space >= &size))
                {
                    if free_index < index {
                        //println!("{index} <-> {free_index}");

                        disk.remove(index);
                        disk.insert(index, DiskEntity::Space { space: size });
                        if let DiskEntity::Space { space } = disk.remove(free_index) {
                            match space.cmp(&size) {
                                Ordering::Greater => {
                                    disk.insert(free_index, DiskEntity::File { size, id });
                                    disk.insert(
                                        free_index + 1,
                                        DiskEntity::Space {
                                            space: space - size,
                                        },
                                    );
                                }
                                Ordering::Equal => {
                                    disk.insert(free_index, DiskEntity::File { size, id });
                                }
                                Ordering::Less => unreachable!(),
                            }
                        }

                        //print_disk(&disk);
                    }
                }
            }
        }
    }

    Ok(disk.checksum())
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
        //assert_eq!(task2, Some(2858));
        assert_eq!(task2, Some(2858));
    }
}
