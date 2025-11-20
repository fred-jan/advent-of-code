extern crate core;

use std::ops::{Add, Mul, Sub};

struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    pub fn ways_to_win(&self) -> u64
    {
        // Analytic solution:
        // d=(1/2)(t +- sqrt(t^2-4s)) where d=delay, t=time, s=displacement
        let disc_sqrt = ((self.time.pow(2) - self.dist.mul(4)) as f64).sqrt();

        // Current record
        let record_max = (self.time as f64 + disc_sqrt) * 0.5;
        let record_min = (self.time as f64 - disc_sqrt) * 0.5;

        // Values to break the record
        let break_min = record_min.add(1.0).floor() as u64;
        let break_max = record_max.sub(1.0).ceil() as u64;

        break_max - break_min + 1
    }
}

struct Puzzle {
    races: Vec<Race>,
}

// Hardcoded inputs (not going to bother writing parsing logic for that amount of records)
impl Puzzle {
    pub fn load_sample_pt1() -> Self {
        Self {
            races: vec![
                Race { time: 7, dist: 9 },
                Race { time: 15, dist: 40 },
                Race { time: 30, dist: 200 },
            ]
        }
    }

    pub fn load_sample_pt2() -> Self {
        Self {
            races: vec![
                Race { time: 71530, dist: 940200 },
            ]
        }
    }

    pub fn load_input_pt1() -> Self {
        Self {
            races: vec![
                Race { time: 47, dist: 282 },
                Race { time: 70, dist: 1079 },
                Race { time: 75, dist: 1147 },
                Race { time: 66, dist: 1062 },
            ]
        }
    }

    pub fn load_input_pt2() -> Self {
        Self {
            races: vec![Race { time: 47707566, dist: 282107911471062 }]
        }
    }

    pub fn ways_to_win(&self) -> u64 {
        self.races.iter().fold(1, |acc, race| acc * race.ways_to_win())
    }
}

fn main() {
    println!("Part 1: {}", Puzzle::load_input_pt1().ways_to_win());
    println!("Part 2: {}", Puzzle::load_input_pt2().ways_to_win());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::load_sample_pt1();

        assert_eq!(Race { time: 7, dist: 9 }.ways_to_win(), 4);
        assert_eq!(Race { time: 15, dist: 40 }.ways_to_win(), 8);
        assert_eq!(Race { time: 30, dist: 200 }.ways_to_win(), 9);
        assert_eq!(puzzle.ways_to_win(), 288);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::load_sample_pt2();

        assert_eq!(puzzle.ways_to_win(), 71503);
    }
}
