use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Stone {
    number: usize,
}

impl Stone {
    fn new(number: usize) -> Self {
        Self { number }
    }

    fn blink(&self) -> Stones {
        if self.number == 0 {
            return Stones::from_slice(&[Self::new(1)]);
        }

        let string = self.number.to_string();

        if string.len() % 2 == 0 {
            let split_at = string.len() / 2;
            return Stones::from_slice(&[
                Self::new(string[0..split_at].parse().unwrap()),
                Self::new(string[split_at..].parse().unwrap()),
            ]);
        }

        Stones::from_slice(&[Self::new(self.number * 2024)])
    }
}

#[derive(Debug)]
struct Stones {
    elements: Vec<Stone>,
}

impl Stones {
    fn from_string(string: &str) -> Self {
        Self {
            elements: string
                .split_whitespace()
                .map(|stone| Stone::new(stone.parse().unwrap()))
                .collect(),
        }
    }

    fn from_slice(list: &[Stone]) -> Self {
        Self {
            elements: list.to_vec(),
        }
    }

    fn blink_count(&self, times: u32) -> usize {
        let mut cache = HashMap::new();
        self.blink_count_cached(times, &mut cache)
    }

    fn blink_count_cached(
        &self,
        blink_times: u32,
        mut cache: &mut HashMap<Stone, HashMap<u32, usize>>,
    ) -> usize {
        self.elements
            .iter()
            .map(|&stone| {
                let cached_stone_counts = cache.entry(stone).or_insert(HashMap::new());

                // Check if this stone has been blinked before this many times, if so re-use count
                if let Some(&cached_count) = cached_stone_counts.get(&blink_times) {
                    return cached_count;
                }

                // Stone has not been blinked this many times, so let's do it!
                let blink_stones = stone.blink();

                // Count and cache the number of stones after blinking (recursive case + special case)
                if blink_times > 1 {
                    let count = blink_stones.blink_count_cached(blink_times - 1, &mut cache);
                    cache.get_mut(&stone).unwrap().insert(blink_times, count);
                    count
                } else {
                    let count = blink_stones.elements.len();
                    cached_stone_counts.insert(blink_times, count);
                    count
                }
            })
            .sum()
    }
}

#[derive(Debug)]
struct Problem {
    stones: Stones,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            stones: Stones::from_string(string),
        }
    }

    fn part_1(&self) -> usize {
        self.stones.blink_count(25)
    }

    fn part_2(&self) -> usize {
        self.stones.blink_count(75)
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day11.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 216996
    println!("Part 2: {}", problem.part_2()); // Attempts: 14090595 (too low), 257335372288947
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"125 17"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(55312, Problem::from_string(SAMPLE).part_1());
    }
}
