use std::{fs, str::FromStr};

struct Problem {
    battery_banks: Vec<BatteryBank>,
}

impl Problem {
    pub fn part_1(&self) -> u32 {
        self.battery_banks
            .iter()
            .map(|bank| bank.largest_joltage_two())
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
    fn largest_joltage_pos(&self) -> usize {
        self.batteries
            .iter()
            .map(|battery| battery.joltage)
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

    fn largest_joltage_two(&self) -> u32 {
        // Do not check last battery to ensure we can pick two subsequent batteries
        let pos_1st = Self {
            batteries: self.batteries[..self.batteries.len() - 1].to_vec(),
        }
        .largest_joltage_pos();

        // println!(
        //     "First max {} at position {}",
        //     self.batteries[pos_1st].joltage, pos_1st
        // );

        // Construct a new bank from the battery after the one with the max joltage to the last
        let pos_2nd = Self {
            batteries: self.batteries[pos_1st + 1..].to_vec(),
        }
        .largest_joltage_pos()
            + pos_1st
            + 1;

        // println!(
        //     "Second max {} at position {}",
        //     self.batteries[pos_2nd].joltage, pos_2nd
        // );

        format!(
            "{}{}",
            self.batteries[pos_1st].joltage, self.batteries[pos_2nd].joltage
        )
        .parse()
        .unwrap()
    }
}

#[derive(Clone)]
struct Battery {
    joltage: u32,
}

impl FromStr for Battery {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            joltage: s
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
}
