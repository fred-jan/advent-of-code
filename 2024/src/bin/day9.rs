use std::fs;

#[derive(Debug)]
struct Filesystem {
    sectors: Vec<Option<u32>>,
}

impl Filesystem {
    fn new() -> Self {
        Self { sectors: vec![] }
    }

    fn compact_sectors(&self) -> Self {
        let files = self.sectors.iter().filter(|sector| sector.is_some());
        let file_count = files.clone().count();

        let mut right = files.rev();

        Self {
            sectors: [
                self.sectors[0..file_count]
                    .iter()
                    .map(|&sector| {
                        match sector {
                            None => *right.next().unwrap(), // Free space -> fill from right
                            Some(_) => sector,
                        }
                    })
                    .collect(),
                vec![None; self.sectors.len() - file_count], // Pad with empty space
            ]
            .concat(),
        }
    }

    fn compact_blocks(&self) -> Self {
        let mut compacted = self.sectors.clone();

        self.sectors
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, Option<u32>)>>()
            .chunk_by(|a, b| a.1 == b.1)
            .filter(|chunk| chunk.iter().all(|(_, sector)| sector.is_some())) // Only files
            .rev() // Start from the end
            .for_each(|file_block| {
                // Search for an empty block fo fit this file in (search space: begin to file start)
                if let Some(empty_block) = compacted[0..file_block[0].0]
                    .iter()
                    .cloned()
                    .enumerate()
                    .collect::<Vec<(usize, Option<u32>)>>()
                    .chunk_by(|a, b| a.1 == b.1)
                    // Only empty blocks
                    .filter(|chunk| chunk.iter().all(|(_, sector)| sector.is_none()))
                    // Find empty block to fit file in
                    .find(|empty_block| empty_block.len() >= file_block.len())
                {
                    // Zip empty and file blocks and switch their values
                    empty_block.iter().zip(file_block).for_each(
                        |((index_empty, _), (index_file, file_sector))| {
                            compacted[*index_empty] = *file_sector;
                            compacted[*index_file] = None;
                        },
                    );
                }
            });

        Self { sectors: compacted }
    }

    fn checksum(&self) -> usize {
        self.sectors
            .iter()
            .enumerate()
            .map(|(i, sector)| i * sector.unwrap_or(0) as usize)
            .sum()
    }
}

#[derive(Debug)]
struct Problem {
    filesystem: Filesystem,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            filesystem: string.lines().nth(0).unwrap().chars().enumerate().fold(
                Filesystem::new(),
                |mut acc, (i, char)| {
                    let size = char.to_digit(10).unwrap() as usize;

                    if i % 2 == 0 {
                        acc.sectors.append(&mut vec![Some(i as u32 / 2); size])
                    } else {
                        acc.sectors.append(&mut vec![None; size]);
                    }

                    acc
                },
            ),
        }
    }

    fn part_1(&self) -> usize {
        self.filesystem.compact_sectors().checksum()
    }

    fn part_2(&self) -> usize {
        self.filesystem.compact_blocks().checksum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day9.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 6211348208140
    println!("Part 2: {}", problem.part_2()); // Attempts: 6239783302560
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_sample_part_1() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(1928, problem.part_1());
    }

    #[test]
    fn test_sample_part_2() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(2858, problem.part_2());
    }
}
