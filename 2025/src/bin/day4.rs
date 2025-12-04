use std::{fs, str::FromStr, vec};

struct Problem {
    diagram: Diagram,
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.diagram.accessible_rolls()
    }
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            diagram: s.parse()?,
        })
    }
}

struct Diagram {
    // row major indexedpositions
    positions: Vec<bool>,
    width: usize,
    height: usize,
}

impl FromStr for Diagram {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            positions: s
                .trim()
                .lines()
                .flat_map(|line| line.chars().map(|char| char == '@'))
                .collect(),
            width: s.trim().find('\n').unwrap(),
            height: s.trim().lines().count(),
        })
    }
}

type Coordinate = (usize, usize);

impl Diagram {
    fn accessible_rolls(&self) -> usize {
        self.positions
            .iter()
            .enumerate()
            .filter(|(_, pos)| **pos)
            .filter(|(i, _)| {
                self.adjacent_positions(i % self.width, i / self.width)
                    .iter()
                    .filter(|(x, y)| self.roll_at(*x, *y))
                    .count() < 4
            })
            .count()
    }

    fn adjacent_positions(&self, x: usize, y: usize) -> Vec<Coordinate> {
        let left = x.checked_sub(1);
        let top = y.checked_sub(1);
        let right = if x < self.width - 1 {
            Some(x + 1)
        } else {
            None
        };
        let bottom = if y < self.height - 1 {
            Some(y + 1)
        } else {
            None
        };

        [
            (left, top),
            (Some(x), top),
            (right, top),
            (left, Some(y)),
            (right, Some(y)),
            (left, bottom),
            (Some(x), bottom),
            (right, bottom),
        ]
        .into_iter()
        .filter_map(|(x_adj, y_adj)| Some((x_adj?, y_adj?)))
        .collect()
    }

    fn roll_at(&self, x: usize, y: usize) -> bool {
        self.positions[x + y * self.width]
    }
}

fn main() {
    let problem = fs::read_to_string("input/day4.txt")
        .expect("Failed to read input")
        .parse::<Problem>()
        .unwrap();

    println!("Part 1: {}", problem.part_1()); // Attempts: 1367
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;
    #[test]
    fn test_diagram() {
        let diagram = SAMPLE.parse::<Problem>().unwrap().diagram;

        assert_eq!(10, diagram.width);
        assert_eq!(10, diagram.height);

        assert!(!diagram.roll_at(0, 0));
        assert!(!diagram.roll_at(1, 0));
        assert!(diagram.roll_at(2, 0));
        assert!(diagram.roll_at(0, 1));
        assert!(!diagram.roll_at(9, 9)); // right bottom
        assert!(diagram.roll_at(8, 9)); // one left of right bottom

        assert_eq!(vec![(1,0), (0,1), (1,1)], diagram.adjacent_positions(0, 0));
        assert_eq!(vec![(8,8), (9,8), (8,9)], diagram.adjacent_positions(9, 9));
    }

    #[test]
    fn test_sample_part_1() {
        assert_eq!(13, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
