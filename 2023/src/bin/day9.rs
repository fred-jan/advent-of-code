use std::fs;

#[derive(Debug, Clone)]
struct Sequence {
    numbers: Vec<i64>,
}

impl Sequence {
    pub fn increments(&self) -> Self {
        Self {
            numbers: self.numbers[0..self.numbers.len() - 1]
                .iter()
                .enumerate()
                .map(|(i, num)| self.numbers[i + 1] - num)
                .collect(),
        }
    }

    pub fn increments_stack(&self) -> Vec<Self> {
        let mut sequences: Vec<Self> = vec![];
        let mut increments = self.increments();
        loop {
            if increments.numbers.iter().all(|&num| num == 0i64) {
                break;
            }

            sequences.push(increments.clone());
            increments = increments.increments();
        }

        sequences
    }

    pub fn next_num(&self) -> i64 {
        self.numbers.last().unwrap()
            + self
                .increments_stack()
                .iter()
                .map(|increments| increments.numbers.last().unwrap())
                .sum::<i64>()
    }

    pub fn prev_num(&self) -> i64 {
        let first_numbers = self
            .increments_stack()
            .iter()
            .map(|increments| increments.numbers.first().unwrap().clone())
            .collect::<Vec<i64>>();

        self.numbers.first().unwrap()
            - first_numbers[0..first_numbers.len() - 1]
                .iter()
                .rfold(first_numbers.last().unwrap().clone(), |acc, num| num - acc)
    }
}

#[derive(Debug)]
struct Report {
    sequences: Vec<Sequence>,
}

impl Report {
    pub fn load(contents: String) -> Self {
        Report {
            sequences: contents
                .lines()
                .map(|line| Sequence {
                    numbers: line
                        .split_whitespace()
                        .map(|number| number.parse::<i64>().expect("Invalid number detected"))
                        .collect::<Vec<i64>>(),
                })
                .collect(),
        }
    }

    pub fn part1(&self) -> i64 {
        self.sequences
            .iter()
            .map(|sequence| sequence.next_num())
            .sum()
    }

    pub fn part2(&self) -> i64 {
        self.sequences
            .iter()
            .map(|sequence| sequence.prev_num())
            .sum()
    }
}

fn main() {
    println!(
        "Part 1: {}",
        Report::load(fs::read_to_string("inputs/day9.txt").unwrap()).part1()
    );
    println!(
        "Part 2: {}",
        Report::load(fs::read_to_string("inputs/day9.txt").unwrap()).part2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        assert_eq!(
            Sequence {
                numbers: vec![0, 3, 6, 9, 12, 15]
            }
            .increments()
            .numbers,
            vec![3, 3, 3, 3, 3]
        );
        assert_eq!(
            Sequence {
                numbers: vec![3, 3, 3, 3, 3]
            }
            .increments()
            .numbers,
            vec![0, 0, 0, 0]
        );
        assert_eq!(
            Sequence {
                numbers: vec![0, 3, 6, 9, 12, 15]
            }
            .next_num(),
            18
        );
        assert_eq!(
            Sequence {
                numbers: vec![10, 13, 16, 21, 30, 45]
            }
            .next_num(),
            68
        );
        assert_eq!(
            Sequence {
                numbers: vec![0, 3, 6, 9, 12, 15]
            }
            .prev_num(),
            -3
        );
        assert_eq!(
            Sequence {
                numbers: vec![0, 3, 6, 9, 12, 15]
            }
            .prev_num(),
            -3
        );
        assert_eq!(
            Sequence {
                numbers: vec![1, 3, 6, 10, 15, 21]
            }
            .prev_num(),
            0
        );
        assert_eq!(
            Sequence {
                numbers: vec![10, 13, 16, 21, 30, 45]
            }
            .prev_num(),
            5
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Report::load(fs::read_to_string("samples/day9.txt").unwrap()).part1(),
            114
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Report::load(fs::read_to_string("samples/day9.txt").unwrap()).part2(),
            2
        );
    }
}
