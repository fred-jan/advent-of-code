use std::{error::Error, fs, str::FromStr, vec};

struct Problem {
    diagram: Diagram,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            diagram: s.trim().parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.diagram.split_count()
    }
}

// Location represented by tuple of Cartesian coordinates (x,y)
type Location = (usize, usize);

struct Diagram {
    start: Location,
    splitters: Vec<Location>,
    width: usize,
    height: usize,
}

impl FromStr for Diagram {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            start: (
                s.lines()
                    .nth(0)
                    .ok_or("No first line")?
                    .find("S")
                    .ok_or("No start position")?,
                0,
            ),
            splitters: s
                .lines()
                .skip(1)
                .enumerate()
                .flat_map(|(j, l)| {
                    l.chars()
                        .enumerate()
                        .filter_map(|(i, c)| match c {
                            '^' => Some((i, j)),
                            _ => None,
                        })
                        .collect::<Vec<Location>>()
                })
                .collect(),
            width: s.find('\n').ok_or("No newline found")?,
            height: s.lines().count(),
        })
    }
}

impl Diagram {
    fn split_count(&self) -> usize {
        let mut beam_xs = vec![self.start.0];
        let mut split_count = 0;

        for j in 1..self.height {
            let splitter_xs = self
                .splitters
                .iter()
                .copied()
                .filter(|loc| loc.1 == j)
                .map(|loc| loc.0)
                .collect::<Vec<usize>>();

            if splitter_xs.is_empty() {
                continue;
            }

            beam_xs = beam_xs
                .iter()
                .flat_map(|beam_x| {
                    // Check wheter the beam hits a splitter
                    if !splitter_xs.contains(beam_x) {
                        return vec![*beam_x];
                    }

                    split_count += 1;

                    // Split this beam into 2 new beams (or 1 if near edge)
                    let mut new_beam_xs = Vec::with_capacity(2);
                    if beam_x.checked_sub(1).is_some() {
                        new_beam_xs.push(beam_x - 1);
                    }
                    if beam_x + 1 < self.width {
                        new_beam_xs.push(beam_x + 1);
                    }
                    new_beam_xs
                })
                .collect::<Vec<usize>>();

            // Deduplicate splits into same column
            beam_xs.sort();
            beam_xs.dedup();
        }

        split_count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day7.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 1622

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(21, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
