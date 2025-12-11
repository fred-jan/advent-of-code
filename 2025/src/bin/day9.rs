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
    from: Point,
    to: Point,
}

impl Line {
    fn from_to(from: Point, to: Point) -> Self {
        Self { from, to }
    }

    fn intersects(&self, other: &Line) -> bool {
        let (ax, ay, bx, by, cx, cy, dx, dy) = (
            self.from.x as f32,
            self.from.y as f32,
            self.to.x as f32,
            self.to.y as f32,
            other.from.x as f32,
            other.from.y as f32,
            other.to.x as f32,
            other.to.y as f32,
        );

        let det = (dx - cx) * (by - ay) - (dy - cy) * (bx - ax);

        if det == 0.0 {
            // Parallel
            return false;
        }

        // t parameterizes the length of AB
        let t = ((dx - cx) * (cy - ay) - (dy - cy) * (cx - ax)) / det;

        if t <= 0.0 || t > 1.0 {
            // Only count at most one length of AB, no more
            return false;
        }

        // u parameterizes the length of AB
        let u = ((bx - ax) * (cy - ay) - (by - ay) * (cx - ax)) / det;

        if u <= 0.0 || u > 1.0 {
            // And only count at most one length of CD
            return false;
        }

        true
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn from_extrema(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    fn inset_one(&self) -> Self {
        Self {
            p1: Point::new(self.p1.x.min(self.p2.x) + 1, self.p1.y.min(self.p2.y) + 1),
            p2: Point::new(self.p1.x.max(self.p2.x) - 1, self.p1.y.max(self.p2.y) - 1),
        }
    }

    fn area(&self) -> u64 {
        (self.p1.x.abs_diff(self.p2.x) + 1) * (self.p1.y.abs_diff(self.p2.y) + 1)
    }

    fn edges(&self) -> [Line; 4] {
        [
            Line::from_to(self.p1, Point::new(self.p1.x, self.p2.y)),
            Line::from_to(Point::new(self.p1.x, self.p2.y), self.p2),
            Line::from_to(self.p2, Point::new(self.p2.x, self.p1.y)),
            Line::from_to(Point::new(self.p2.x, self.p1.y), self.p1),
        ]
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
            .map(|window| Line::from_to(window[0], window[1]))
            .collect();

        let first_line_start = boundaries.first().expect("Expected last line").from;
        let last_line_end = boundaries.last().expect("Expected last line").to;
        let closing_line = Line::from_to(last_line_end, first_line_start);

        // Connect the last and first lines
        boundaries.push(closing_line);

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

                let rect = Rectangle::from_extrema(*tile_a, *tile_b);

                // Inset the rectangle by one so edges formed from (parts of) the boundary lines do
                // no longer touch those. Then check if any of the edges of the new rectangle
                // intersect with any of the boundary lines. Any such intersection indicates that
                // the rectangle is not wholly contained by the outline.
                let rect_inset_edges = rect.inset_one().edges();
                let rect_intersects_boundary = boundaries.iter().any(|boundary| {
                    rect_inset_edges
                        .iter()
                        .any(|rect_edge| rect_edge.intersects(boundary))
                });

                areas.insert(
                    (tile_b, tile_a),
                    if rect_intersects_boundary {
                        None
                    } else {
                        Some(rect.area())
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
    println!("Part 2: {}", problem.part_2()); // Attempts: 1644094530

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
    fn test_rectangle() {
        let rect = Rectangle::from_extrema(Point::new(0, 0), Point::new(4, 4));

        assert_eq!(25, rect.area());
        assert_eq!(16, rect.inset_one().area());

        let rect = Rectangle::from_extrema(Point::new(4, 4), Point::new(0, 0));

        assert_eq!(16, rect.inset_one().area());
    }

    #[test]
    fn test_sample_part_1() {
        assert_eq!(50, SAMPLE.parse::<Problem>().unwrap().part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(24, SAMPLE.parse::<Problem>().unwrap().part_2());
    }
}
