use std::{error::Error, fs, str::FromStr};

struct Problem {
    diagram: Diagram,
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.diagram.accessible_rolls()
    }

    pub fn part_2(&self) -> usize {
        self.diagram.accessible_rolls_recursive()
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

#[derive(Clone)]
struct Diagram {
    // row major indexed positions
    locations: Vec<bool>,
    width: usize,
    height: usize,
}

impl FromStr for Diagram {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            locations: s
                .trim()
                .lines()
                .flat_map(|line| line.chars().map(|c| c == '@'))
                .collect(),
            width: s.trim().find('\n').ok_or("No newline found")?,
            height: s.trim().lines().count(),
        })
    }
}

// Location represented by tuple of Cartesian coordinates (x,y)
type Location = (usize, usize);

impl Diagram {
    fn accessible_roll_locs(&self) -> Vec<Location> {
        self.locations
            .iter()
            .enumerate()
            .filter(|(_, has_paper)| **has_paper) // Only locations with paper rolls
            .map(|(i, _)| self.idx_to_loc(i)) // Turn row major index into cartesian coordinate
            .filter(|loc| {
                // Only take locations that have less than four adjacent paper rolls
                self.adjacent_locs(loc.0, loc.1)
                    .into_iter()
                    .filter(|loc| self.locations[self.loc_to_idx(*loc)])
                    .count()
                    < 4
            })
            .collect()
    }

    fn accessible_rolls(&self) -> usize {
        self.accessible_roll_locs().len()
    }

    fn accessible_rolls_recursive(&self) -> usize {
        let mut diagram = self.clone();
        let mut accessible_rolls = 0;

        loop {
            let locs = diagram.accessible_roll_locs();

            if locs.is_empty() {
                break;
            }

            accessible_rolls += locs.len();

            // Remove all accessible paper rolls
            for loc in locs {
                let idx = diagram.loc_to_idx(loc);
                diagram.locations[idx] = false;
            }
        }

        accessible_rolls
    }

    #[rustfmt::skip]
    fn adjacent_locs(&self, x: usize, y: usize) -> Vec<Location> {
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

        // Define all adjacent locations with None components when invalid (eg. x < 0 or y > height)
        [
            (left, top), (Some(x), top), (right, top),
            (left, Some(y)), (right, Some(y)),
            (left, bottom), (Some(x), bottom), (right, bottom),
        ]
        .into_iter()
        // Remove all locations having a none component
        .filter_map(|(x_adj, y_adj)| Some((x_adj?, y_adj?)))
        .collect()
    }

    fn loc_to_idx(&self, loc: Location) -> usize {
        loc.0 + loc.1 * self.width
    }

    fn idx_to_loc(&self, idx: usize) -> Location {
        (idx % self.width, idx / self.width)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day4.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 1367
    println!("Part 2: {}", problem.part_2()); // Attempts: 9144

    Ok(())
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

        assert_eq!(vec![(1, 0), (0, 1), (1, 1)], diagram.adjacent_locs(0, 0));
        assert_eq!(vec![(8, 8), (9, 8), (8, 9)], diagram.adjacent_locs(9, 9));
    }

    #[test]
    fn test_sample_part_1() {
        assert_eq!(13, SAMPLE.parse::<Problem>().unwrap().part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(43, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
