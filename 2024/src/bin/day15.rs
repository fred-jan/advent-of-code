use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, Mul};

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

impl Mul<i32> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
struct Map {
    dimensions: Vec2i,
    boxes: HashSet<Vec2i>,
    walls: HashSet<Vec2i>,
    robot_pos: Vec2i,
    directions: Vec<Vec2i>,
}

impl Map {
    fn from_string(string: &str) -> Self {
        let (top, bottom) = string.split_once("\n\n").unwrap();

        let map_string = top
            .replace("#\n#", "\n")
            .trim_matches('#')
            .trim_matches('\n')
            .to_string();
        let width = map_string.find("\n").unwrap_or(map_string.len());

        let objects = map_string
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
            dimensions: Vec2i::new(width as i32, map_string.lines().count() as i32),
            boxes: objects.get(&'O').unwrap().clone(),
            walls: objects.get(&'#').unwrap().clone(),
            robot_pos: *objects.get(&'@').unwrap().iter().nth(0).unwrap(),
            directions: bottom
                .split_whitespace()
                .flat_map(|line| {
                    line.chars().map(|char| match char {
                        '^' => Vec2i::new(0, -1),
                        '>' => Vec2i::new(1, 0),
                        'v' => Vec2i::new(0, 1),
                        '<' => Vec2i::new(-1, 0),
                        _ => panic!("Unknown direction"),
                    })
                })
                .collect(),
        }
    }

    fn is_accessible(&self, pos: Vec2i) -> bool {
        pos.x >= 0
            && pos.x < self.dimensions.x
            && pos.y >= 0
            && pos.y < self.dimensions.y
            && !self.walls.contains(&pos)
    }

    fn move_robot(&self) -> Self {
        let mut boxes = self.boxes.clone();
        let mut robot_pos = self.robot_pos.clone();

        'move_loop: for &direction in self.directions.iter() {
            // println!(
            //     "Trying to move robot at {} in direction {}",
            //     robot_pos, direction
            // );

            let mut position;
            let mut boxes_to_move = vec![];
            for i in 1.. {
                position = robot_pos + (direction * i);

                if !self.is_accessible(position) {
                    // println!("    Position {} is not accessible", position);
                    // Out of bounds, can't move in this direction so check next direction
                    continue 'move_loop;
                }

                if boxes.contains(&position) {
                    // println!("    Box found at {}", position);

                    // Box, so queue its movement and check next position
                    boxes_to_move.push(position);
                    continue;
                }

                // println!("    Free spot found at {}", position);
                // Free spot found, so break the loop and perform the queued movements
                break;
            }

            // Perform movements
            for &box_pos in boxes_to_move.iter().rev() {
                boxes.remove(&box_pos); // remove from old space
                boxes.insert(box_pos + direction); // insert into new pos

                // println!("    Moving box from {} to {}", box_pos, box_pos + direction);
            }

            // println!(
            //     "    Moving robot from {} to {}",
            //     robot_pos,
            //     robot_pos + direction
            // );

            // Once the boxes are moved, we can move the robot
            robot_pos = robot_pos + direction;
        }

        Self {
            dimensions: self.dimensions,
            walls: self.walls.clone(),
            boxes,
            robot_pos,
            directions: self.directions.clone(),
        }
    }

    fn box_gps_sum(&self) -> u32 {
        self.boxes
            .iter()
            .map(|box_pos| (box_pos.x + 1 + (box_pos.y + 1 ) * 100) as u32)
            .sum()
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
        // dbg!(&self.map);

        self.map.move_robot().box_gps_sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day15.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts:
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const SAMPLE_SIMPLIFIED: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(2028, Problem::from_string(SAMPLE_SIMPLIFIED).part_1());
        assert_eq!(10092, Problem::from_string(SAMPLE).part_1());
    }
}
