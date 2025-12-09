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
    pub fn part_1(&self) -> u64 {
        self.floor_plan.largest_rect_area()
    }

    pub fn part_2(&self) -> u64 {
        self.floor_plan.largest_rect_area_constrained()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn contains(&self, loc: &Point) -> bool {
        (self.start.x.min(self.end.x)..=self.start.x.max(self.end.x)).contains(&loc.x)
            && (self.start.y.min(self.end.y)..=self.start.y.max(self.end.y)).contains(&loc.y)
    }

    fn cross_2d(&self) -> i64 {
        (self.start.x * self.end.y) as i64 - (self.end.x * self.start.y) as i64
    }
}

struct FloorPlan {
    red_tiles: Vec<Point>,
}

impl FromStr for FloorPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            red_tiles: s
                .lines()
                .map(|l| match l.split_once(',') {
                    Some((x, y)) => Ok(Point {
                        x: x.parse()
                            .map_err(|_| format!("Invalid X coordinate: {}", x))?,
                        y: y.parse()
                            .map_err(|_| format!("Invalid Y coordinate: {}", y))?,
                    }),
                    None => Err("Unable to parse coordinate".to_string()),
                })
                .collect::<Result<Vec<Point>, _>>()?,
        })
    }
}

impl FloorPlan {
    fn largest_rect_area(&self) -> u64 {
        let mut areas = HashMap::new();

        for tile_a in self.red_tiles.iter() {
            for tile_b in self.red_tiles.iter() {
                if tile_a == tile_b {
                    continue;
                }

                // Check if same rectangle was already calculated from reverse ordered pair
                if areas.contains_key(&(tile_b, tile_a)) {
                    continue;
                }

                // Area: (|b_x-a_x| + 1) * (|b_y - a_y| + 1), the one accounts for the unit width
                let area = (tile_b.x.abs_diff(tile_a.x) + 1) * (tile_b.y.abs_diff(tile_a.y) + 1);

                areas.insert((tile_b, tile_a), area);
            }
        }

        let mut sorted_areas: Vec<u64> = areas.into_values().collect();
        sorted_areas.sort_by(|a, b| b.cmp(a));
        sorted_areas[0]
    }

    fn largest_rect_area_constrained(&self) -> u64 {
        let mut boundaries: Vec<Line> = self
            .red_tiles
            .windows(2)
            .map(|window| Line {
                start: window[0],
                end: window[1],
            })
            .collect();

        let first_line_start = boundaries.first().expect("Expected last line").start;
        let last_line_end = boundaries.last().expect("Expected last line").end;
        let closing_line = Line {
            start: last_line_end,
            end: first_line_start,
        };

        // Connect the last and first lines
        boundaries.push(closing_line);

        // Shoelace formula to determine winding order (sum of 2d cross products > 0 with Y down -> CW)
        let result: i64 = boundaries.iter().map(|line| line.cross_2d()).sum();

        let fill_start = if result > 0 {
            if first_line_start.x == last_line_end.x {
                // Vertical line (same X)
                Point::new(first_line_start.x + 1, first_line_start.y + 1)
            } else {
                // Horizontal line (same Y)
                Point::new(first_line_start.x + 1, first_line_start.y - 1)
            }
        } else if first_line_start.x == last_line_end.x {
            // Vertical line (same X)
            Point::new(first_line_start.x + 1, first_line_start.y - 1)
        } else {
            // Horizontal line (same Y)
            Point::new(first_line_start.x - 1, first_line_start.y - 1)
        };

        // Flood fill to determine the inside locations
        let mut inside = vec![];
        let mut to_visit = vec![fill_start];
        while let Some(loc) = to_visit.pop() {
            // Skip if already visited or lies on a boundary line
            if inside.contains(&loc) || boundaries.iter().any(|line| line.contains(&loc)) {
                continue;
            }

            // Record this location as inside location
            inside.push(loc);

            // Visit adjacent positions (left, right, top, bottom)
            to_visit.append(&mut vec![
                Point::new(loc.x - 1, loc.y),
                Point::new(loc.x + 1, loc.y),
                Point::new(loc.x, loc.y - 1),
                Point::new(loc.x, loc.y + 1),
            ]);
        }

        let mut areas = HashMap::new();

        for tile_a in self.red_tiles.iter() {
            for tile_b in self.red_tiles.iter() {
                if tile_a == tile_b {
                    continue;
                }

                // Check if we already evaluated these tiles in reverse order
                if areas.contains_key(&(tile_b, tile_a)) {
                    continue;
                }

                // Corners of the rectangle in CW winding order
                let corners = [
                    Point::new(tile_a.x, tile_a.y),
                    Point::new(tile_b.x, tile_a.y),
                    Point::new(tile_b.x, tile_b.y),
                    Point::new(tile_a.x, tile_b.y),
                ];

                // Calculate the rectangle area if its corners lie on a boundary or inside,
                // otherwise register the result as None to indicate the rectangle is not
                // applicable (prevents processing of the same rectangle from reverse order tiles)
                areas.insert(
                    (tile_b, tile_a),
                    if corners.iter().all(|corner| {
                        inside.contains(corner)
                            || boundaries.iter().any(|line| line.contains(corner))
                    }) {
                        Some((tile_b.x.abs_diff(tile_a.x) + 1) * (tile_b.y.abs_diff(tile_a.y) + 1))
                    } else {
                        None
                    },
                );
            }
        }

        let mut sorted_areas: Vec<u64> = areas.into_values().flatten().collect();
        sorted_areas.sort_by(|a, b| b.cmp(a));
        sorted_areas[0]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day9.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // Attempts: 4737026542 (too low), 4737096935
    println!("Part 2: {}", problem.part_2()); // Attempts: 

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

    #[test]
    fn test_sample_part_2() {
        assert_eq!(24, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
