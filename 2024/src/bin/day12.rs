use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_adjacent(&self, other: Vec2) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        dx == 0 && dy == 1 || dx == 1 && dy == 0
    }
}

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    positions: HashSet<Vec2>,
}

impl Region {
    fn new(plant: char) -> Self {
        Self {
            plant,
            positions: HashSet::new(),
        }
    }

    fn perimeter(&self) -> u32 {
        let shared_borders: u32 = self
            .positions
            .iter()
            .enumerate()
            .map(|(index, &pos)| {
                self.positions
                    .iter()
                    .skip(index + 1)
                    .filter(|&&other| pos != other && pos.is_adjacent(other))
                    .count() as u32
            })
            .sum();

        self.positions.len() as u32 * 4 - 2 * shared_borders
    }

    /// Returns the number of corners the region has, which is the same as the number of sides
    fn sides(&self) -> u32 {
        self.positions
            .iter()
            // Create hashmap mapping each of the four corners of a plant to the plant that touches
            // this corner. Corner coordinates are expanded from the plant coordinate, so plant
            // (x,y) has corners (x,y), (x,y+1), (x+1,y), (x+1,y+1).
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Vec2, Vec<Vec2>>, &plant_pos| {
                    acc.entry(plant_pos).or_insert(vec![]).push(plant_pos);
                    acc.entry(Vec2::new(plant_pos.x, plant_pos.y + 1))
                        .or_insert(vec![])
                        .push(plant_pos);
                    acc.entry(Vec2::new(plant_pos.x + 1, plant_pos.y))
                        .or_insert(vec![])
                        .push(plant_pos);
                    acc.entry(Vec2::new(plant_pos.x + 1, plant_pos.y + 1))
                        .or_insert(vec![])
                        .push(plant_pos);

                    acc
                },
            )
            .iter()
            .map(|(_, plants)| match plants.len() {
                // Count corners shared with only one plant or by three plants 1, these respectively
                // represent the outside and inside corners of the region.
                1 | 3 => 1,
                // Corners shared by two plants should be counted twice as an outside corner if
                // those plants are diagonal to each other (this is the special case of diagonal
                // regions that was mentioned in the instructions).
                2 => {
                    if plants[0].x.abs_diff(plants[1].x) == 1
                        && plants[0].y.abs_diff(plants[1].y) == 1
                    {
                        2
                    } else {
                        0
                    }
                }
                4 | _ => 0,
            })
            .sum()
    }

    fn area(&self) -> u32 {
        self.positions.len() as u32
    }

    fn price(&self) -> u32 {
        self.perimeter() * self.area()
    }

    fn discounted_price(&self) -> u32 {
        self.sides() * self.area()
    }

    fn is_adjacent_pos(&self, pos: Vec2) -> bool {
        self.positions
            .iter()
            .find(|plant_pos| plant_pos.is_adjacent(pos))
            .is_some()
    }
}

#[derive(Debug)]
struct Plot {
    width: u32,
    plants: Vec<char>,
}

impl Plot {
    fn from_string(string: &str) -> Self {
        Self {
            width: string.find("\n").unwrap_or(string.len()) as u32,
            plants: string.lines().flat_map(|line| line.chars()).collect(),
        }
    }

    fn regions(&self) -> Vec<Region> {
        let regions: Vec<Region> =
            self.plants
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut regions, (index, &plant)| {
                    let pos = Vec2::new(
                        (index as u32 % self.width) as i32,
                        (index as u32 / self.width) as i32,
                    );

                    let adjacent_regions: Vec<(usize, Region)> = regions
                        .iter()
                        .enumerate()
                        .filter(|(_, region)| region.plant == plant && region.is_adjacent_pos(pos))
                        .map(|(index, region)| (index, region.clone()))
                        .collect();

                    let mut region = if adjacent_regions.len() == 0 {
                        Region::new(plant)
                    } else {
                        // Multiple adjacent regions to join, so the plant is effectively connecting two
                        // or more region into one new big region. So let's remove the old regions first.
                        adjacent_regions.iter().rev().for_each(|(index, _)| {
                            regions.remove(*index);
                        });

                        // Next create one big new region containing all plants of previous regions
                        Region {
                            plant,
                            positions: adjacent_regions
                                .iter()
                                .flat_map(|(_, adjacent_region)| adjacent_region.positions.clone())
                                .collect(),
                        }
                    };

                    region.positions.insert(pos);
                    regions.push(region);

                    regions
                });

        regions
    }
}

#[derive(Debug)]
struct Problem {
    plot: Plot,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            plot: Plot::from_string(string),
        }
    }

    fn part_1(&self) -> u32 {
        self.plot
            .regions()
            .iter()
            .map(|region| region.price())
            .sum()
    }

    fn part_2(&self) -> u32 {
        self.plot
            .regions()
            .iter()
            .map(|region| region.discounted_price())
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day12.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 1449902
    println!("Part 2: {}", problem.part_2()); // Attempts: 908042
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SIMPLIFIED_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const SAMPLE_SIMPLIFIED2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const SAMPLE: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const SAMPLE_SIMPLIFIED3: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    const SAMPLE_SIMPLIFIED4: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(140, Problem::from_string(SAMPLE_SIMPLIFIED_1).part_1());
        assert_eq!(772, Problem::from_string(SAMPLE_SIMPLIFIED2).part_1());
        assert_eq!(1930, Problem::from_string(SAMPLE).part_1());
    }
    #[test]
    fn test_sample_part_2() {
        assert_eq!(80, Problem::from_string(SAMPLE_SIMPLIFIED_1).part_2());
        assert_eq!(436, Problem::from_string(SAMPLE_SIMPLIFIED2).part_2());
        assert_eq!(236, Problem::from_string(SAMPLE_SIMPLIFIED3).part_2());
        assert_eq!(368, Problem::from_string(SAMPLE_SIMPLIFIED4).part_2());
        assert_eq!(1206, Problem::from_string(SAMPLE).part_2());
    }

    #[test]
    fn test_region_sides() {
        assert_eq!(4, Plot::from_string("AAA").regions()[0].sides());
        assert_eq!(4, Plot::from_string("A\nA\nA").regions()[0].sides());
        assert_eq!(4, Plot::from_string("AA\nAA").regions()[0].sides());
        assert_eq!(6, Plot::from_string("AA\nA").regions()[0].sides());
        assert_eq!(6, Plot::from_string("AAA\nA").regions()[0].sides());

        let plot = Plot::from_string(
            "RRRR..\n\
             ..RRR.\n\
             ..R...",
        );
        assert_eq!('R', plot.regions()[1].plant);
        assert_eq!(10, plot.regions()[1].sides());
    }
}
