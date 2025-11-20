use std::fs;

struct Problem {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Problem {
    fn from_string(string: String) -> Self {
        let (left, right) = string
            .lines()
            .map(|line| {
                let mut ws = line.split_whitespace();
                (
                    ws.next().unwrap().parse::<u32>().unwrap(),
                    ws.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .unzip();

        Self { left, right }
    }

    fn part_1(&self) -> u32 {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        left.sort();
        right.sort();

        left.iter()
            .zip(right.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum()
    }

    fn part_2(&self) -> u32 {
        self.left
            .iter()
            .map(|&left| left * self.right.iter().filter(|&&right| left == right).count() as u32)
            .sum()
    }
}

fn main() {
    let problem =
        Problem::from_string(fs::read_to_string("input/day1.txt").expect("Failed to read input"));

    println!("Part 1: {}", problem.part_1());
    println!("Part 2: {}", problem.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            11,
            Problem::from_string(
                "3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3"
                    .to_string(),
            )
            .part_1()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            31,
            Problem::from_string(
                "3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3"
                    .to_string(),
            )
            .part_2()
        );
    }
}
