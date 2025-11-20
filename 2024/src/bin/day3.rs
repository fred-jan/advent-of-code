use std::fs;

struct Problem {
    instructions: String,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            instructions: string.to_string(),
        }
    }

    fn process(&self, instructions: &String) -> usize {
        instructions
            .match_indices("mul(")
            .filter_map(|(start, _)| {
                let offset = start + 4; // length of "mul("
                instructions[offset..(offset + instructions[offset..].find(")")?)].split_once(',')
            })
            .filter(|(left_operand, right_operand)| {
                left_operand.chars().all(|char| char.is_ascii_digit())
                    && right_operand.chars().all(|char| char.is_ascii_digit())
            })
            .map(|(left_operand, right_operand)| {
                left_operand.parse::<usize>().unwrap() * right_operand.parse::<usize>().unwrap()
            })
            .sum()
    }

    fn part_1(&self) -> usize {
        self.process(&self.instructions)
    }

    fn part_2(&self) -> usize {
        self.process(
            &self
                .instructions
                .split("don't()")
                .enumerate()
                // Take right of each part, effectively removing section from don't() to do()
                .map(|(i, part)| match part.split_once("do()") {
                    None => match i {
                        0 => part, // Always include the first part (assume do())
                        _ => "",
                    },
                    Some((_, right)) => right,
                })
                .collect::<String>(),
        )
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day3.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 185797128
    println!("Part 2: {}", problem.part_2()); // Attempts: 144809740 (too high), 89798695
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(
            161,
            Problem::from_string(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
            .part_1()
        );
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(
            48,
            Problem::from_string(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )
            .part_2()
        );
    }
}
