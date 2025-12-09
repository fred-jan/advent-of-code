use std::{collections::HashMap, error::Error, fs, str::FromStr};

struct Problem {
    floor_plan: FloorPlan,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            floor_plan: s.trim().parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.floor_plan.largest_rectangle_area()
    }
}

// 2D Cartesian coordinates (x,y)
type Location = (usize, usize);

struct FloorPlan {
    red_tiles: Vec<Location>,
}

impl FromStr for FloorPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            red_tiles: s
                .lines()
                .map(|l| match l.split_once(',') {
                    Some((x, y)) => Ok((
                        x.parse()
                            .map_err(|_| format!("Invalid X coordinate: {}", x))?,
                        y.parse()
                            .map_err(|_| format!("Invalid Y coordinate: {}", y))?,
                    )),
                    None => Err("Unable to parse coordinate".to_string()),
                })
                .collect::<Result<Vec<Location>, _>>()?,
        })
    }
}

impl FloorPlan {
    fn largest_rectangle_area(&self) -> usize {
        let mut areas = HashMap::new();

        for tile_a in self.red_tiles.iter() {
            for tile_b in self.red_tiles.iter() {
                if tile_a == tile_b {
                    continue;
                }

                if areas.contains_key(&(tile_a, tile_b)) || areas.contains_key(&(tile_b, tile_a)) {
                    continue;
                }

                // Area: (|b_x-a_x| + 1) * (|b_y - a_y| + 1), the one accounts for the unit width
                let area = (tile_b.0.abs_diff(tile_a.0) + 1) * (tile_b.1.abs_diff(tile_a.1) + 1);

                areas.insert((tile_a, tile_b), area);
                areas.insert((tile_b, tile_a), area);
            }
        }

        let mut sorted_areas: Vec<usize> = areas.into_values().collect();
        sorted_areas.sort();
        sorted_areas.sort_by(|a, b| b.cmp(a));
        sorted_areas[0]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day9.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 4737026542 (too low), 4737096935

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(50, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
