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
        self.inventory.fresh_available_ingredients()
    }

    pub fn part_2(&self) -> u64 {
        self.inventory.fresh_ingredients()
    }
}

struct Inventory {
    ingredient_ranges: Vec<(u64, u64)>,
    available_ingredients: Vec<u64>,
}

impl Inventory {
    fn fresh_available_ingredients(&self) -> usize {
        self.available_ingredients
            .iter()
            .filter(|ingredient| {
                for range in &self.ingredient_ranges {
                    if (range.0..=range.1).contains(ingredient) {
                        return true;
                    }
                }
                false
            })
            .count()
    }

    fn fresh_ingredients(&self) -> u64 {
        self.ingredient_ranges
            .iter()
            .fold(vec![], |mut result_ranges, &range| {
                let (mut start, mut end) = range;
                let mut result_range_indices_to_remove = vec![];

                for (i, &(result_start, result_end)) in result_ranges.iter().enumerate() {
                    // Check whether the endpoints of the range are contained within the already
                    // processed result ranges. If so, trim the range to remove overlaps.
                    if (result_start..=result_end).contains(&start) {
                        start = result_end + 1;
                    }
                    if (result_start..=result_end).contains(&end) {
                        end = result_start - 1;
                    }

                    // Range is fully contained in already processed ranges, skip it
                    if start > end {
                        return result_ranges;
                    }

                    // Range fully contains this already processed range, so mark the already
                    // processed range to be removed from the resulting ranges
                    if (start..=end).contains(&result_start) && (start..=end).contains(&result_end)
                    {
                        result_range_indices_to_remove.push(i);
                    }
                }

                if start <= end {
                    result_ranges.push((start, end));
                }

                for i in result_range_indices_to_remove {
                    result_ranges.swap_remove(i);
                }

                result_ranges
            })
            .into_iter()
            .map(|(result_start, result_end)| result_end - result_start + 1)
            .sum()
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
            ingredient_ranges: range_lines
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
            available_ingredients: ingredient_lines
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

    println!("Part 1: {}", problem.part_1()); // Attempts: 739
    println!("Part 2: {}", problem.part_2()); // Attempts: 355636767906941 (too high), 344486348901788

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

    #[test]
    fn test_sample_part_2() {
        assert_eq!(14, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
