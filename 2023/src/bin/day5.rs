use std::fs;

#[derive(Debug)]
struct Map {
    _name: String,
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn convert(&self, from: u64) -> u64 {
        match self
            .entries
            .iter()
            .find(|map_entry| map_entry.is_applicable(from))
        {
            Some(map_entry) => map_entry.convert(from),
            None => from,
        }
    }
}

#[derive(Debug)]
struct MapEntry {
    source_start: u64,
    dest_start: u64,
    range: u64,
}

impl MapEntry {
    pub fn is_applicable(&self, from: u64) -> bool {
        from >= self.source_start && from < self.source_start + self.range
    }

    pub fn convert(&self, from: u64) -> u64 {
        if !self.is_applicable(from) {
            return from;
        }

        self.dest_start + from - self.source_start
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn load(contents: String) -> Self {
        let (seeds, maps) = contents.split_once("\n\n").unwrap();

        Self {
            seeds: seeds[7..]
                .split_ascii_whitespace()
                .map(|seed| seed.parse::<u64>().unwrap())
                .collect(),
            maps: maps
                .split("\n\n")
                .map(|map_section| Map {
                    _name: map_section.lines().next().unwrap().to_string(),
                    entries: map_section
                        .lines()
                        .skip(1)
                        .map(|line| {
                            let nums: Vec<u64> = line
                                .split_ascii_whitespace()
                                .map(|num| num.parse::<u64>().unwrap())
                                .collect();

                            MapEntry {
                                dest_start: nums[0],
                                source_start: nums[1],
                                range: nums[2],
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub fn to_location(&self, seed: &u64) -> u64 {
        self.maps
            .iter()
            .fold(seed.clone(), |acc, map| map.convert(acc))
    }

    pub fn pt1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.to_location(seed))
            .min()
            .unwrap()
    }

    pub fn pt2(&self) -> u64 {
        // 1815746760 seeds o_O
        self.seeds
            .chunks(2)
            .into_iter()
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .flatten()
            .map(|seed| self.to_location(&seed))
            .min()
            .unwrap()
    }
}

fn main() {
    let almanac =
        Almanac::load(fs::read_to_string("inputs/day5.txt").expect("Failed to read input"));
    println!("Part 1: {}", almanac.pt1());
    println!("Part 2: {}", almanac.pt2());
}

#[test]
fn test_common() {
    assert_eq!(
        MapEntry {
            dest_start: 52,
            source_start: 50,
            range: 48
        }
        .convert(53),
        55
    );
    assert_eq!(
        MapEntry {
            dest_start: 52,
            source_start: 50,
            range: 48
        }
        .convert(10),
        10
    );
    assert_eq!(
        MapEntry {
            dest_start: 52,
            source_start: 50,
            range: 48
        }
        .convert(97),
        99
    );
    assert_eq!(
        MapEntry {
            dest_start: 52,
            source_start: 50,
            range: 48
        }
        .convert(98),
        98
    );
}

#[test]
fn test_part1() {
    let almanac =
        Almanac::load(fs::read_to_string("samples/day5.txt").expect("Failed to read input"));

    assert_eq!(almanac.to_location(&79), 82);
    assert_eq!(almanac.to_location(&14), 43);
    assert_eq!(almanac.to_location(&55), 86);
    assert_eq!(almanac.to_location(&13), 35);
    assert_eq!(almanac.pt1(), 35);
}

#[test]
fn test_part2() {
    let almanac =
        Almanac::load(fs::read_to_string("samples/day5.txt").expect("Failed to read input"));

    assert_eq!(almanac.pt2(), 46);
}
