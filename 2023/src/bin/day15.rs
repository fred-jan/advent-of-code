use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Step {
    line: String,
}

impl Step {
    pub fn load(input: &str) -> Self {
        Self {
            line: input.to_string(),
        }
    }

    pub fn operation(&self) -> Operation {
        Operation::load(&self.line)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Remove { label: String },
    Insert { label: String, focal_length: u8 },
}

impl Operation {
    pub fn load(input: &str) -> Self {
        match input.find('=') {
            Some(index) => Self::Insert {
                label: input[..index].to_string(),
                focal_length: input.chars().last().unwrap().to_digit(10).unwrap() as u8,
            },
            None => Self::Remove {
                label: input[..input.len() - 1].to_string(),
            },
        }
    }
}

#[derive(Debug)]
struct Sequence {
    steps: Vec<Step>,
}

impl Sequence {
    pub fn load(contents: String) -> Self {
        Self {
            steps: contents
                .trim()
                .split(',')
                .map(|line| Step::load(line))
                .collect(),
        }
    }

    pub fn hash(&self, input: &str) -> u8 {
        input
            .chars()
            .fold(0, |acc, char| ((acc + char as u16) * 17) % 256) as u8
    }

    pub fn part1(&self) -> usize {
        self.steps
            .iter()
            .fold(0, |acc, step| acc + self.hash(&step.line) as usize)
    }

    pub fn part2(&self) -> usize {
        let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();

        // Install lenses by processing steps
        self.steps.iter().for_each(|step| match step.operation() {
            Operation::Remove { label } => {
                if let Some(b) = boxes.get_mut(&self.hash(&label)) {
                    b.retain(|(lens_label, _)| lens_label != &label);
                }
            }
            Operation::Insert {
                label,
                focal_length,
            } => {
                let b = boxes.entry(self.hash(&label)).or_insert(vec![]);

                match b.iter().position(|(lens_label, _)| lens_label == &label) {
                    Some(pos) => b[pos] = (label, focal_length),
                    None => b.push((label, focal_length)),
                };
            }
        });

        // Add up focusing power of the lenses
        boxes.iter().fold(0, |acc, (i_box, b)| {
            acc + b
                .iter()
                .enumerate()
                .fold(0, |acc_box, (i_slot, (_, focal_length))| {
                    acc_box + (*i_box as usize + 1) * (i_slot + 1) * *focal_length as usize
                })
        })
    }
}

fn main() {
    let platform = Sequence::load(fs::read_to_string("inputs/day15.txt").unwrap());

    // 514394
    println!("Part 1: {}", platform.part1());

    println!("Part 2: {}", platform.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sequence = Sequence::load(fs::read_to_string("samples/day15.txt").unwrap());

        assert_eq!(sequence.hash("HASH"), 52);
        assert_eq!(sequence.hash("rn=1"), 30);
        assert_eq!(sequence.hash("cm-"), 253);
        assert_eq!(sequence.hash("qp=3"), 97);
        assert_eq!(sequence.hash("cm=2"), 47);
        assert_eq!(sequence.hash("qp-"), 14);
        assert_eq!(sequence.hash("pc=4"), 180);
        assert_eq!(sequence.hash("ot=9"), 9);
        assert_eq!(sequence.hash("ab=5"), 197);
        assert_eq!(sequence.hash("pc-"), 48);
        assert_eq!(sequence.hash("pc=6"), 214);
        assert_eq!(sequence.hash("ot=7"), 231);

        assert_eq!(sequence.part1(), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Step::load("rn=1").operation(),
            Operation::Insert {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
        assert_eq!(
            Step::load("cm-").operation(),
            Operation::Remove {
                label: "cm".to_string(),
            }
        );

        assert_eq!(
            Sequence::load(fs::read_to_string("samples/day15.txt").unwrap()).part2(),
            145
        );
    }
}
