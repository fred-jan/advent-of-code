use std::{error::Error, fs, str::FromStr};

struct Problem {
    homework: Homework,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            homework: s.trim().parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> u64 {
        self.homework.grand_total()
    }
}

struct Homework {
    matrix: Matrix,
    operators: Vec<Operator>,
}

impl FromStr for Homework {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (numbers, operators) = s.rsplit_once("\n").ok_or("No last line")?;

        Ok(Self {
            matrix: numbers.parse::<Matrix>()?,
            operators: operators
                .split_whitespace()
                .map(|operator| operator.parse::<Operator>())
                .collect::<Result<Vec<Operator>, _>>()?,
        })
    }
}

impl Homework {
    fn grand_total(&self) -> u64 {
        self.operators
            .iter()
            .enumerate()
            .map(|(i, operator)| operator.apply(self.matrix.column(i)))
            .sum()
    }
}

enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply(&self, operands: Vec<u64>) -> u64 {
        operands
            .into_iter()
            .reduce(|result, number| match self {
                Self::Add => result + number,
                Self::Mul => result * number,
            })
            .expect("Unable to apply operator")
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().nth(0).ok_or("Expected operator char")? {
            '+' => Self::Add,
            '*' => Self::Mul,
            c => Err(format!("Unexpected operator char: {}", c))?,
        })
    }
}

struct Matrix {
    rows: Vec<Vec<u64>>,
}

impl Matrix {
    fn column(&self, i: usize) -> Vec<u64> {
        self.rows.iter().map(|row| row[i]).collect()
    }
}

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(|l| {
                    Ok::<Vec<u64>, String>(
                        l.split_whitespace()
                            .map(|number| number.parse::<u64>().map_err(|_| "Expected number"))
                            .collect::<Result<Vec<u64>, _>>()?,
                    )
                })
                .collect::<Result<Vec<Vec<u64>>, _>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day6.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 4719804927602

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(4277556, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
