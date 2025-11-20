use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Add<i64> for Vec2 {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl Machine {
    /// Using solution to the corresponding system of equations
    fn prize_combination(&self) -> Option<(u64, u64)> {
        let discriminant = self.button_b.x * self.button_a.y - self.button_a.x * self.button_b.y;

        if discriminant == 0 {
            // Divide by zero -> no solution
            return None;
        }

        let numerator_a = self.button_b.x * self.prize.y - self.button_b.y * self.prize.x;
        let numerator_b = self.button_a.x * self.prize.y - self.button_a.y * self.prize.x;

        if numerator_a % discriminant > 0 || numerator_b % discriminant != 0 {
            // Only accept integer solutions
            return None;
        }

        Some((
            (numerator_a / discriminant) as u64,
            (numerator_b / -discriminant) as u64,
        ))
    }

    fn correct_prize(&self) -> Self {
        Self {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: self.prize + 10_000_000_000_000,
        }
    }
}

#[derive(Debug)]
struct Problem {
    machines: Vec<Machine>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            machines: string
                .split("\n\n")
                .map(|machine_string| {
                    let mut machine_lines = machine_string.lines();

                    let (ax, ay) = machine_lines.next().unwrap()[12..]
                        .split_once(", Y+")
                        .unwrap();
                    let (bx, by) = machine_lines.next().unwrap()[12..]
                        .split_once(", Y+")
                        .unwrap();
                    let (px, py) = machine_lines.next().unwrap()[9..]
                        .split_once(", Y=")
                        .unwrap();

                    Machine {
                        button_a: Vec2 {
                            x: ax.parse().unwrap(),
                            y: ay.parse().unwrap(),
                        },
                        button_b: Vec2 {
                            x: bx.parse().unwrap(),
                            y: by.parse().unwrap(),
                        },
                        prize: Vec2 {
                            x: px.parse().unwrap(),
                            y: py.parse().unwrap(),
                        },
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(|machine| machine.prize_combination())
            .filter(|(times_a, times_b)| *times_a <= 100 && *times_b <= 100)
            .map(|(times_a, times_b)| times_a * 3 + times_b)
            .sum()
    }

    fn part_2(&self) -> u64 {
        self.machines
            .iter()
            .map(|machine| machine.correct_prize())
            .filter_map(|machine| machine.prize_combination())
            .map(|(times_a, times_b)| times_a * 3 + times_b)
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day13.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 38487 (too high), 36838
    println!("Part 2: {}", problem.part_2()); // Attempts: 83029436920891
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    const SAMPLE_SIMPLIFIED1: &str = r#"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(0, Problem::from_string(SAMPLE_SIMPLIFIED1).part_1());
        assert_eq!(480, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(875318608908, Problem::from_string(SAMPLE).part_2());
    }
}
