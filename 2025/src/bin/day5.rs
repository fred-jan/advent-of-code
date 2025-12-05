use std::{error::Error, fs, str::FromStr};

struct Problem {
    inventory: Inventory,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inventory: s.parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.inventory.fresh_ingredients()
    }
}

struct Inventory {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Inventory {
    fn fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|ingredient| {
                for range in &self.ranges {
                    if (range.0..=range.1).contains(ingredient) {
                        return true;
                    }
                }
                false
            })
            .count()
    }
}

impl FromStr for Inventory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (range_lines, ingredient_lines) = s
            .trim()
            .split_once("\n\n")
            .ok_or("Missing blank line in input")?;

        Ok(Self {
            ranges: range_lines
                .lines()
                .map(|range_line| {
                    let (start, end) = range_line
                        .split_once('-')
                        .ok_or("Missing range separator")?;

                    Ok((
                        start
                            .parse::<u64>()
                            .map_err(|_| "Range start is not an integer")?,
                        end.parse::<u64>()
                            .map_err(|_| "Range end is not an integer")?,
                    ))
                })
                .collect::<Result<Vec<(u64, u64)>, String>>()?,
            ingredients: ingredient_lines
                .lines()
                .map(|line| {
                    line.parse::<u64>()
                        .map_err(|_| "Ingredient is not an integer")
                })
                .collect::<Result<Vec<u64>, _>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day5.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(3, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
