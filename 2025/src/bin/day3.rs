use std::{fs, str::FromStr};

struct Problem {
    battery_banks: Vec<BatteryBank>,
}

impl Problem {
    pub fn part_1(&self) -> u64 {
        self.battery_banks
            .iter()
            .map(|bank| bank.largest_joltage_n(2))
            .sum()
    }

    pub fn part_2(&self) -> u64 {
        self.battery_banks
            .iter()
            .map(|bank| bank.largest_joltage_n(12))
            .sum()
    }
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            battery_banks: s
                .trim()
                .lines()
                .map(|line| line.parse::<BatteryBank>())
                .collect::<Result<Vec<BatteryBank>, _>>()?,
        })
    }
}

struct BatteryBank {
    batteries: Vec<Battery>,
}

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s
                .trim()
                .chars()
                .map(|char| char.to_string().parse::<Battery>())
                .collect::<Result<Vec<Battery>, _>>()?,
        })
    }
}

impl BatteryBank {
    fn max_rating_index(&self) -> usize {
        self.batteries
            .iter()
            .map(|battery| battery.rating)
            .enumerate()
            .reduce(|el_max, el_current| {
                if el_current.1 > el_max.1 {
                    el_current
                } else {
                    el_max
                }
            })
            .unwrap()
            .0
    }

    fn largest_joltage_n(&self, n: usize) -> u64 {
        let mut pick_offset = 0;
        let mut pick_ratings: Vec<u32> = Vec::with_capacity(n);

        // Pick exactly n battery ratings from the bank
        for i in 0..n {
            // We need to pick n-i subsequent ratings, so check all up to the (n-i)th (inclusive)
            let n_to_pick = n - i;

            // println!(
            //     "Picking from range [{}..={}]",
            //     pick_offset,
            //     self.batteries.len() - n_to_pick
            // );

            let bank = Self {
                batteries: self.batteries[pick_offset..=self.batteries.len() - n_to_pick].to_vec(),
            };

            // Determine the index of the battery with the max joltage rating
            let i_bank_max = bank.max_rating_index();

            // Add the corresponding joltage rating to the ratings picked so far
            pick_ratings.push(bank.batteries[i_bank_max].rating);

            // Update offset so that subsequent batteries are picked after the last chosen one
            pick_offset += i_bank_max + 1;

            // println!(
            //     "{}: max {} at position {}",
            //     i,
            //     bank.batteries[i_bank_max].rating,
            //     pick_offset - 1,
            // );
        }

        // Construct the joltage from the picked joltage ratings
        pick_ratings
            .iter()
            .map(|rating| rating.to_string())
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

#[derive(Clone)]
struct Battery {
    rating: u32,
}

impl FromStr for Battery {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rating: s
                .trim()
                .chars()
                .nth(0)
                .ok_or("No joltage value found")?
                .to_digit(10)
                .ok_or("Invalid joltage value given")?,
        })
    }
}

fn main() {
    let problem = fs::read_to_string("input/day3.txt")
        .expect("Failed to read input")
        .parse::<Problem>()
        .unwrap();

    println!("Part 1: {}", problem.part_1()); // Attempts: 17330, 17493
    println!("Part 2: {}", problem.part_2()); // Attempts: 173685428989126
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(357, SAMPLE.parse::<Problem>().unwrap().part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(3121910778619, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
