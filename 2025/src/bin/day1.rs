use std::fs;

struct Problem {
    safe: Safe,
}

impl Problem {
    pub fn from_string(string: &str) -> Self {
        Self {
            safe: Safe::from_string(string),
        }
    }

    pub fn part_1(&self) -> u32 {
        self.safe.password()
    }
}

struct Safe {
    dial: Dial,
    rotations: Vec<Rotation>,
}

impl Safe {
    pub fn from_string(string: &str) -> Self {
        Self {
            dial: Dial::default(),
            rotations: string.lines().map(Rotation::from_string).collect(),
        }
    }

    pub fn password(&self) -> u32 {
        let (_dial, times_zero) = self.rotations.iter().fold(
            (self.dial.clone(), 0),
            |(dial, mut times_zero), rotation| {
                let dial = dial.interact(rotation);

                if dial.position == 0 {
                    times_zero += 1;
                }

                (dial, times_zero)
            },
        );

        times_zero
    }
}

#[derive(Clone)]
struct Dial {
    size: u32,
    position: u32,
}

impl Dial {
    pub fn interact(&self, rotation: &Rotation) -> Self {
        Self {
            size: self.size,
            position: match rotation {
                Rotation::Left(distance) => {
                    (self.position + self.size - (distance % self.size)) % self.size
                }
                Rotation::Right(distance) => (self.position + distance) % self.size,
            },
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            size: 100,
            position: 50,
        }
    }
}

enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    pub fn from_string(string: &str) -> Self {
        let mut chars = string.chars();

        let direction = chars.next().expect("Missing direction letter");
        let distance = chars
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse rotation distance as unsigned int");

        match direction {
            'L' => Self::Left(distance),
            'R' => Self::Right(distance),
            _ => panic!("Unsupported rotation direction"),
        }
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day1.txt")
            .expect("Failed to read input")
            .as_str(),
    );
    println!("Part 1: {}", problem.part_1()); // Attempts: 33, 989
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(3, Problem::from_string(SAMPLE).part_1());
    }
}
