use std::{error::Error, fs, str::FromStr, vec};

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

    pub fn part_2(&self) -> u64 {
        self.homework.grand_total_rtl()
    }
}

struct Homework {
    matrix: Matrix,
    operators: Vec<Operator>,
    worksheet: String,
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
            worksheet: s.to_string(),
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

    // Totally different approach, just reach character width columns from right to left
    // constructing numbers from top to bottom digits until an operator is found. Once the operator
    // is found the constructed numbers are processed according to the found operator.
    fn grand_total_rtl(&self) -> u64 {
        let (numbers, operators) = self.worksheet.rsplit_once("\n").expect("No last line");

        // Iterate from last char-width column to the first
        (0..numbers.lines().nth(0).unwrap().len())
            .rev()
            .fold((0u64, vec![]), |(result, mut stack), j| {
                // Construct number by assembling present digits from first row to the last
                let number = numbers
                    .lines()
                    .filter_map(|line| line.chars().nth(j))
                    .collect::<String>()
                    .trim()
                    .parse::<u64>();

                // If a number was constructed, add it to the stack to be processed
                match number {
                    Ok(number) => stack.push(number),
                    Err(_) => return (result, stack), // whitespace column
                }

                // Check if an operator is found, if so process the stack using that operator
                match operators
                    .chars()
                    .nth(j)
                    .unwrap_or(' ')
                    .to_string()
                    .parse::<Operator>()
                {
                    Ok(operator) => (result + operator.apply(stack), vec![]),
                    Err(_) => (result, stack),
                }
            })
            .0
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
                        // TODO: split on specific index and preserve whitespace for aligntment
                        // also represent each number as string (or right pad with zeroes)
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
    println!("Part 2: {}", problem.part_2()); // Attempts: 9608327000261

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
    fn test_matrix() {
        let matrix = SAMPLE.parse::<Problem>().unwrap().homework.matrix;

        assert_eq!(vec![64, 23, 314], matrix.column(3));
    }

    #[test]
    fn test_sample_part_1() {
        assert_eq!(4277556, SAMPLE.parse::<Problem>().unwrap().part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(3263827, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
