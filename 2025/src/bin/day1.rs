use std::{fmt::Display, fs};

struct Problem {
    safe: Safe,
}

impl Problem {
    pub fn from_string(string: &str) -> Self {
        Self {
            safe: Safe::from_string(string.trim()),
        }
    }

    pub fn part_1(&self) -> u32 {
        self.safe.do_rotations().zero_ends
    }

    pub fn part_2(&self) -> u32 {
        self.safe.do_rotations().zero_clicks
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

    pub fn do_rotations(&self) -> Dial {
        self.rotations
            .iter()
            .fold(self.dial.clone(), |dial, rotation| dial.interact(rotation))
    }
}

#[derive(Clone)]
struct Dial {
    size: u32,
    position: u32,
    zero_clicks: u32,
    zero_ends: u32,
}

impl Dial {
    pub fn interact(&self, rotation: &Rotation) -> Self {
        // Full revolutions
        let revolutions = rotation.distance / self.size;

        // Remainder distance
        let rem_distance = rotation.distance % self.size;

        // Net distance to be added (normalizes direction)
        let add_distance = match rotation.direction {
            // Subtracting D mod X is the same as adding X-D mod X
            Direction::Left => self.size - rem_distance,
            Direction::Right => rem_distance,
        };

        let new_position = (self.position + add_distance) % self.size;

        let extra_rev = self.position != 0
            && match rotation.direction {
                Direction::Left => new_position > self.position,
                Direction::Right => new_position < self.position,
            };

        let mut zero_clicks = self.zero_clicks + revolutions;

        // Add zero click ending at zero, or when starting from non-zero and making a revolution
        if new_position == 0 || extra_rev {
            zero_clicks += 1;
        }

        // println!(
        //     "Old pos: {}, Rotation: {}, New pos: {}, Revolutions: {}, Zero clicks: {}",
        //     self.position, rotation, new_position, revolutions, zero_clicks
        // );

        Self {
            size: self.size,
            position: new_position,
            zero_clicks,
            zero_ends: self.zero_ends + if new_position == 0 { 1 } else { 0 },
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            size: 100,
            position: 50,
            zero_clicks: 0,
            zero_ends: 0,
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_char(chr: &char) -> Self {
        match chr {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unsupported rotation direction"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => 'L',
                Self::Right => 'R',
            }
        )
    }
}

struct Rotation {
    direction: Direction,
    distance: u32,
}

impl Rotation {
    pub fn from_string(string: &str) -> Self {
        let mut chars = string.chars();

        Self {
            direction: Direction::from_char(&chars.next().expect("Missing direction letter")),
            distance: chars
                .as_str()
                .parse::<u32>()
                .expect("Unable to parse rotation distance as unsigned int"),
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day1.txt")
            .expect("Failed to read input")
            .as_str(),
    );
    println!("Part 1: {}", problem.part_1()); // Attempts: 33, 989
    println!("Part 2: {}", problem.part_2()); // Attempts: 5949, 5941
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(3, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(6, Problem::from_string(SAMPLE).part_2());
    }
}
