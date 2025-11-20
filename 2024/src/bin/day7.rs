use std::fs;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
struct Equation {
    outcome: usize,
    operands: Vec<usize>,
}

impl Equation {
    fn from_string(string: &str) -> Self {
        let (left, right) = string.split_once(": ").unwrap();

        Self {
            outcome: left.parse().unwrap(),
            operands: right
                .split_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect(),
        }
    }

    fn is_solvable(&self, operators: &[Operator]) -> bool {
        operators
            .iter()
            .find(|operator| {
                let evaluated = match operator {
                    Operator::Add => self.operands[0] + self.operands[1],
                    Operator::Mul => self.operands[0] * self.operands[1],
                    Operator::Concat => format!("{}{}", self.operands[0], self.operands[1])
                        .parse()
                        .unwrap(),
                };

                // If these were the last operands, compare without expected outcome
                if self.operands.len() == 2 {
                    return evaluated == self.outcome;
                }

                // Otherwise replace the two evaluated operands with their result and recurse
                let new_eqn = Equation {
                    outcome: self.outcome,
                    operands: [vec![evaluated], self.operands[2..].to_vec()].concat(),
                };
                new_eqn.is_solvable(operators)
            })
            .is_some()
    }
}

#[derive(Debug)]
struct Problem {
    equations: Vec<Equation>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            equations: string
                .lines()
                .map(|line| Equation::from_string(line))
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        self.equations
            .iter()
            .filter(|eqn| eqn.is_solvable(&[Operator::Add, Operator::Mul]))
            .map(|eqn| eqn.outcome)
            .sum()
    }

    fn part_2(&self) -> usize {
        self.equations
            .iter()
            .filter(|eqn| eqn.is_solvable(&[Operator::Add, Operator::Mul, Operator::Concat]))
            .map(|eqn| eqn.outcome)
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day7.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 538191549061
    println!("Part 2: {}", problem.part_2()); // Attempts: 34612812972206
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_sample_part_1() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(3749, problem.part_1());
    }

    #[test]
    fn test_sample_part_2() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(11387, problem.part_2());
    }

    #[test]
    fn test_equation_is_solvable() {
        let operators = [Operator::Add, Operator::Mul];
        assert_eq!(
            true,
            Equation::from_string("190: 10 19").is_solvable(&operators)
        );
        assert_eq!(
            true,
            Equation::from_string("3267: 81 40 27").is_solvable(&operators)
        );
        assert_eq!(
            false,
            Equation::from_string("21037: 9 7 18 13").is_solvable(&operators)
        );
    }
}
