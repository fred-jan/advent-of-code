use std::{collections::HashMap, error::Error, fs, str::FromStr, vec};

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
        self.diagram.count_splits()
    }

    pub fn part_2(&self) -> usize {
        self.diagram
            .count_timelines(self.diagram.start, &mut HashMap::new())
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
    fn count_timelines(
        &self,
        loc_particle: Location,
        loc_counts: &mut HashMap<Location, usize>,
    ) -> usize {
        // Check the cache to prevent computing the timelines for this particle position again
        if let Some(timeline_count) = loc_counts.get(&loc_particle) {
            return *timeline_count;
        }

        for j in loc_particle.1..self.height {
            // Find splitter hit (if any) on current row, otherwise go to next row
            match self
                .row_splitter_xs(j)
                .into_iter()
                .find(|splitter_x| *splitter_x == loc_particle.0)
            {
                // Splitter hit by the particle, count timelines on traversable branches
                Some(_) => {
                    let mut timeline_count = 0;

                    if loc_particle.0.checked_sub(1).is_some() {
                        timeline_count +=
                            self.count_timelines((loc_particle.0 - 1, j + 1), loc_counts)
                    }
                    if loc_particle.0 + 1 < self.width {
                        timeline_count +=
                            self.count_timelines((loc_particle.0 + 1, j + 1), loc_counts)
                    }

                    // Cache the number of timelines for this particle position
                    loc_counts.insert(loc_particle, timeline_count);

                    return timeline_count;
                }
                // If no splitter was hit by the particle, traverse to next row
                None => continue,
            };
        }

        // End reached, count as one timeline
        1
    }

    fn count_splits(&self) -> usize {
        let mut beam_xs = vec![self.start.0];
        let mut split_count = 0;

        for j in 1..self.height {
            let splitter_xs = self.row_splitter_xs(j);

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

    fn row_splitter_xs(&self, j: usize) -> Vec<usize> {
        self.splitters
            .iter()
            .copied()
            .filter(|loc| loc.1 == j)
            .map(|loc| loc.0)
            .collect::<Vec<usize>>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day7.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 1622
    println!("Part 2: {}", problem.part_2()); // Attempts: 10357305916520

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

    #[test]
    fn test_sample_part_2() {
        assert_eq!(40, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
