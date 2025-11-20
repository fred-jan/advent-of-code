use std::collections::{HashMap, HashSet};
use std::fs;

type Position = (u32, u32);

struct Directions {
    steps: HashMap<Position, HashSet<Position>>,
}

impl Directions {
    fn count_paths(&self, from: Position, to: Position) -> usize {
        match self.steps.get(&from) {
            None => 0,
            Some(next_positions) => {
                if next_positions.contains(&to) {
                    1
                } else {
                    next_positions
                        .iter()
                        .map(|&next_pos| self.count_paths(next_pos, to))
                        .sum()
                }
            }
        }
    }

    fn leads_to(&self, goal: Position) -> bool {
        self.steps
            .iter()
            .find(|(_, to)| to.contains(&goal))
            .is_some()
    }
}

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    heights: Vec<u8>,
}

impl Map {
    fn from_string(string: &str) -> Map {
        Self {
            width: string.find("\n").unwrap() as u32,
            height: string.lines().count() as u32,
            heights: string
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap_or(127) as u8)
                })
                .collect(),
        }
    }

    fn positions_of_height(&self, of_height: u8) -> Vec<Position> {
        self.heights
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == of_height)
            .map(|(i, _)| (i as u32 % self.width, i as u32 / self.width))
            .collect()
    }

    fn trailheads(&self) -> Vec<Position> {
        self.positions_of_height(0)
    }

    fn peaks(&self) -> Vec<Position> {
        self.positions_of_height(9)
    }

    fn neighbours_of(&self, pos: Position) -> Vec<Position> {
        let mut neighbours = vec![];
        if pos.0 > 0 {
            neighbours.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.width - 1 {
            neighbours.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            neighbours.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.height - 1 {
            neighbours.push((pos.0, pos.1 + 1));
        }
        neighbours
    }

    fn directions_from(&self, from: Position) -> Directions {
        let mut steps = HashMap::new();
        let mut frontier = vec![from];
        while let Some(current_pos) = frontier.pop() {
            let height = self.heights[(current_pos.0 + current_pos.1 * self.width) as usize];
            self.neighbours_of(current_pos)
                .iter()
                .filter(|(x, y)| self.heights[(x + y * self.width) as usize] == height + 1) // Only heights one higher than current
                .for_each(|neighbour_pos| {
                    frontier.push(*neighbour_pos);
                    steps
                        .entry(current_pos)
                        .or_insert(HashSet::new())
                        .insert(*neighbour_pos);
                });
        }

        Directions { steps }
    }
}

#[derive(Debug)]
struct Problem {
    map: Map,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            map: Map::from_string(string),
        }
    }

    fn part_1(&self) -> usize {
        let peaks = self.map.peaks();

        self.map
            .trailheads()
            .iter()
            .map(|trailhead_pos| {
                let trails = self.map.directions_from(*trailhead_pos);
                // For each trailhead check if directions to this peak are available (= reachable)
                peaks
                    .iter()
                    .filter(|peak_pos| trails.leads_to(**peak_pos))
                    .count()
            })
            .sum()
    }

    fn part_2(&self) -> usize {
        let peaks = self.map.peaks();

        self.map
            .trailheads()
            .iter()
            .map(|trailhead_pos| {
                let trails = self.map.directions_from(*trailhead_pos);
                peaks
                    .iter()
                    .map(|peak_pos| trails.count_paths(*trailhead_pos, *peak_pos))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day10.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 822
    println!("Part 2: {}", problem.part_2()); // Attempts: 1801
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PART1_SIMPLIFIED1: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    const SAMPLE_PART1_SIMPLIFIED2: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    const SAMPLE_PART2_SIMPLIFIED1: &str = r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#;

    const SAMPLE_PART2_SIMPLIFIED2: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    const SAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(2, Problem::from_string(SAMPLE_PART1_SIMPLIFIED1).part_1());
        assert_eq!(4, Problem::from_string(SAMPLE_PART1_SIMPLIFIED2).part_1());
        assert_eq!(36, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(3, Problem::from_string(SAMPLE_PART2_SIMPLIFIED1).part_2());
        assert_eq!(13, Problem::from_string(SAMPLE_PART2_SIMPLIFIED2).part_2());
        assert_eq!(81, Problem::from_string(SAMPLE).part_2());
    }

    #[test]
    fn test_map_neighbours() {
        let map = Map::from_string(SAMPLE);
        assert_eq!(vec![(1, 0), (0, 1)], map.neighbours_of((0, 0)));
        assert_eq!(
            vec![(0, 1), (2, 1), (1, 0), (1, 2)],
            map.neighbours_of((1, 1))
        );
        assert_eq!(vec![(6, 7), (7, 6)], map.neighbours_of((7, 7)));
    }
}
