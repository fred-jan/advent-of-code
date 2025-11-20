use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};

#[derive(Eq, Debug, Copy, Clone, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

impl Sub<Position> for Position {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Position>>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            width: string.find("\n").unwrap(),
            height: string.lines().count(),
            antennas: string
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, char)| match char {
                            '.' => None,
                            _ => Some((char, Position::new(x as isize, y as isize))),
                        })
                })
                .fold(HashMap::new(), |mut acc, (char, position)| {
                    acc.entry(char).or_insert(vec![]).push(position);
                    acc
                }),
        }
    }

    fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0
            && position.y >= 0
            && (position.x as usize) < self.width
            && (position.y as usize) < self.height
    }

    fn part_1(&self) -> usize {
        self.antennas
            .iter()
            .flat_map(|(&_char, positions)| {
                positions
                    .iter()
                    .enumerate()
                    .flat_map(move |(i, &antenna1)| {
                        positions[i + 1..].iter().flat_map(move |&antenna2| {
                            let delta = antenna2 - antenna1;
                            [antenna1 - delta, antenna2 + delta]
                                .into_iter()
                                .filter(|antinode| self.in_bounds(*antinode))
                        })
                    })
            })
            .collect::<HashSet<Position>>() // Deduplicate antinode positions
            .iter()
            .count()
    }

    fn part_2(&self) -> usize {
        self.antennas
            .iter()
            .flat_map(|(&_char, positions)| {
                positions
                    .iter()
                    .enumerate()
                    .flat_map(move |(i, &antenna1)| {
                        positions[i + 1..].iter().flat_map(move |&antenna2| {
                            let delta = antenna2 - antenna1;
                            let mut antinodes = vec![];

                            // Extrapolate backward
                            let mut antinode = antenna1;
                            while self.in_bounds(antinode) {
                                antinodes.push(antinode.clone());
                                antinode = antinode - delta;
                            }

                            // Extrapolate forward
                            antinode = antenna2;
                            while self.in_bounds(antinode) {
                                antinodes.push(antinode.clone());
                                antinode = antinode + delta;
                            }

                            antinodes
                        })
                    })
            })
            .collect::<HashSet<Position>>() // Deduplicate antinode positions
            .iter()
            .count()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day8.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    // Attempts: 304 (too high), 291 (too high), 293 (too high), 299 (too high), 280
    println!("Part 1: {}", problem.part_1());
    println!("Part 2: {}", problem.part_2()); // Attempt: 958
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_sample_part_1() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(14, problem.part_1());
    }

    #[test]
    fn test_sample_part_2() {
        let problem = Problem::from_string(SAMPLE);

        assert_eq!(34, problem.part_2());
    }
}
