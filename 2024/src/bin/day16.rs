use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Vec2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TileState {
    position: Vec2i,
    direction: Vec2i,
    points: u32,
}

impl Ord for TileState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.points.cmp(&self.points)
    }
}

impl PartialOrd for TileState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Map {
    walls: HashSet<Vec2i>,
    start: Vec2i,
    finish: Vec2i,
}

impl Map {
    fn from_string(string: &str) -> Self {
        let objects = string
            .split_whitespace()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, char)| (char, Vec2i::new(x as i32, y as i32)))
            })
            .fold(HashMap::new(), |mut acc, (char, pos)| {
                acc.entry(char).or_insert(HashSet::new()).insert(pos);
                acc
            });

        Self {
            walls: objects.get(&'#').unwrap().clone(),
            start: *objects.get(&'S').unwrap().iter().nth(0).unwrap(),
            finish: *objects.get(&'E').unwrap().iter().nth(0).unwrap(),
        }
    }

    /// Dijkstra with priority queue. Never used this combination before so learned from this
    /// example: https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html#examples
    fn points_from_to(&self) -> Option<u32> {
        let mut tile_points = HashMap::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(TileState {
            position: self.start,
            direction: Vec2i::new(1, 0),
            points: 0,
        });

        // Pick a tile for which its adjacent tiles should be explored. The priority queue ensures
        // that the tile with the lowest amount of points will be explored first. See TileState::cmp
        while let Some(TileState {
            position,
            direction,
            points,
        }) = frontier.pop()
        {
            // Once we reach the finish return the accumulated points
            if position == self.finish {
                return Some(points);
            }

            // If we already visited this tile with a lower amount of points, skip
            if *tile_points.get(&position).unwrap_or(&u32::MAX) < points {
                continue;
            }

            [
                Vec2i::new(1, 0),
                Vec2i::new(0, 1),
                Vec2i::new(-1, 0),
                Vec2i::new(0, -1),
            ]
            .iter()
            // Determine new position for each direction
            .map(|&adj_dir| (position + adj_dir, adj_dir))
            .filter(|(adj_position, _)| !self.walls.contains(adj_position))
            .for_each(|(adj_position, adj_direction)| {
                // Add 1 point if the direction is unchanged, otherwise add 1001 points
                let adj_points = points + if adj_direction != direction { 1001 } else { 1 };

                // If this tile has not been visited before, or if the points now are less than
                // previous visit, then add the tile with its points to the record.
                if adj_points < *tile_points.get(&adj_position).unwrap_or(&u32::MAX) {
                    tile_points.insert(adj_position, adj_points);
                    // Also add the tile to the frontier so it will be further explored
                    frontier.push(TileState {
                        position: adj_position,
                        direction: adj_direction,
                        points: adj_points,
                    });
                }
            });
        }

        None
    }
}

#[derive(Debug)]
struct Problem {
    map: Map,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            map: Map::from_string(string),
        }
    }

    fn part_1(&self) -> u32 {
        self.map.points_from_to().expect("No solution!")
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day16.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 94444
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    const SAMPLE_SIMPLIFIED: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(7036, Problem::from_string(SAMPLE_SIMPLIFIED).part_1());
        assert_eq!(11048, Problem::from_string(SAMPLE).part_1());
    }
}
