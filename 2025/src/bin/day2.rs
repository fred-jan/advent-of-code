use std::{fmt::format, fs, str::FromStr};

struct Problem {
    ranges: Vec<Range>,
}

impl Problem {
    pub fn part_1(&self) -> u64 {
        self.ranges
            .iter()
            .map(|range| range.sum_invalid_ids())
            .sum()
    }
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ranges: s
                .trim()
                .split(',')
                .map(|range| range.parse::<Range>())
                .collect::<Result<Vec<Range>, _>>()?,
        })
    }
}

struct Range {
    from: u64,
    to: u64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once('-') {
            Some((from, to)) => Ok(Self {
                from: from.parse::<u64>().map_err(|_| format!("Invalid range start: {}", from))?,
                to: to.parse::<u64>().map_err(|_| format!("Invalid range end: {}", to))?,
            }),
            None => Err("Missing hyphen in range".to_string()),
        }
    }
}

impl Range {
    fn sum_invalid_ids(&self) -> u64 {
        let mut sum = 0;

        for id in self.from..=self.to {
            let id_str = id.to_string();
            let length = id_str.len();

            for n in 1..=(length / 2) {
                if length % n != 0 {
                    // Only check the ID if its length divisible by the sequence length, to make sure
                    // that the sequence can be repeated exactly the amount of times to match the
                    // length of the ID
                    continue;
                }

                let max_repeat_count = 2;
                // let max_repeat_count = length / n;

                let seq = id_str
                    .chars()
                    .take(n)
                    .collect::<String>()
                    .repeat(max_repeat_count);

                if id_str == seq {
                    // println!("{} matches {}", id_str, seq);
                    sum += id;
                }
            }
        }

        sum
    }
}

fn main() {
    let problem = fs::read_to_string("input/day2.txt")
        .expect("Failed to read input")
        .parse::<Problem>()
        .unwrap();

    println!("Part 1: {}", problem.part_1()); // Attempts: 20223751480 
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(1227775554, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
