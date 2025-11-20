use std::fmt::Debug;
use std::fs;

#[derive(Debug, Clone)]
struct Record {
    sequence: String,
    groups: Vec<usize>,
}

impl Record {
    pub fn load(contents: &str) -> Self {
        let (sequence, groups) = contents.split_once(' ').unwrap();

        Self {
            sequence: sequence.to_string(),
            groups: groups
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        }
    }

    pub fn is_match(&self) -> bool {
        let broken_spring_parts = self.sequence.split('.').filter(|part| !part.is_empty());

        if broken_spring_parts.clone().count() != self.groups.len() {
            return false;
        }

        broken_spring_parts
            .zip(&self.groups)
            .filter(|(part, &group)| part.len() == group)
            .count()
            == self.groups.len()
    }

    pub fn expand(&self) -> Vec<Self> {
        match self.sequence.chars().position(|char| char == '?') {
            Some(pos) => [
                Self {
                    sequence: [&self.sequence[..pos], ".", &self.sequence[pos + 1..]].concat(),
                    groups: self.groups.clone(),
                }
                .expand(),
                Self {
                    sequence: [&self.sequence[..pos], "#", &self.sequence[pos + 1..]].concat(),
                    groups: self.groups.clone(),
                }
                .expand(),
            ]
            .concat(),
            None => vec![self.clone()],
        }
    }

    pub fn arrangements(&self) -> usize {
        self.expand()
            .iter()
            // .inspect(|record| {
            //     println!("{}: {}", record.sequence, record.is_match());
            // })
            .filter(|record| record.is_match())
            .count()
    }
}

#[derive(Debug)]
struct Puzzle {
    records: Vec<Record>,
}

impl Puzzle {
    pub fn load(contents: String) -> Self {
        Self {
            records: contents.lines().map(|line| Record::load(line)).collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.records
            .iter()
            .fold(0, |acc, record| acc + record.arrangements())
    }
}

fn main() {
    let puzzle = Puzzle::load(fs::read_to_string("inputs/day12.txt").unwrap());

    // 7857
    println!("Part 1: {}", puzzle.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Record::load("???.### 1,1,3").arrangements(), 1);
        assert_eq!(Record::load(".??..??...?##. 1,1,3").arrangements(), 4);
        assert_eq!(Record::load("?#?#?#?#?#?#?#? 1,3,1,6").arrangements(), 1);
        assert_eq!(Record::load("????.#...#... 4,1,1").arrangements(), 1);
        assert_eq!(Record::load("????.######..#####. 1,6,5").arrangements(), 4);
        assert_eq!(Record::load("?###???????? 3,2,1").arrangements(), 10);

        let sequence = Puzzle::load(fs::read_to_string("samples/day12.txt").unwrap());

        assert_eq!(sequence.part1(), 21);
    }
}
