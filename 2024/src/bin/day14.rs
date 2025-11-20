use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, Div, Mul, Rem};

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

impl Mul<i32> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Vec2i {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Vec2i::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vec2i> for Vec2i {
    type Output = Self;
    fn div(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Rem<Vec2i> for Vec2i {
    type Output = Self;
    fn rem(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Vec2i,
    velocity: Vec2i,
}

impl Robot {
    fn elapse_time(&self, seconds: u32, area_dims: Vec2i) -> Self {
        let mut position = (self.position + self.velocity * seconds as i32) % area_dims;

        // Process boundary wrapping (teleports)
        position = (position + area_dims) % area_dims;

        Self {
            position,
            velocity: self.velocity,
        }
    }

    /// Iterative approach since I'm too lazy to lookup how to determine modular inverses
    fn repeat_interval(&self, area_dims: Vec2i) -> u32 {
        let mut robot = self.clone();
        let mut previous_pos = HashSet::new();
        (1..)
            .find(|_| {
                previous_pos.insert(robot.position);

                robot = robot.elapse_time(1, area_dims);

                if !previous_pos.contains(&robot.position) {
                    return false;
                }
                return true;
            })
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Area {
    dimensions: Vec2i,
    robots: Vec<Robot>,
}

impl Area {
    fn from_string(width: u32, height: u32, string: &str) -> Self {
        Self {
            dimensions: Vec2i::new(width as i32, height as i32),
            robots: string
                .lines()
                .map(|line| line.split_once(" ").unwrap())
                .map(|(left, right)| {
                    let (px, py) = left[2..].split_once(",").unwrap();
                    let (vx, vy) = right[2..].split_once(",").unwrap();

                    Robot {
                        position: Vec2i::new(px.parse().unwrap(), py.parse().unwrap()),
                        velocity: Vec2i::new(vx.parse().unwrap(), vy.parse().unwrap()),
                    }
                })
                .collect(),
        }
    }

    fn elapse_time(&self, seconds: u32) -> Self {
        Self {
            dimensions: self.dimensions,
            robots: self
                .robots
                .iter()
                .map(|robot| robot.elapse_time(seconds, self.dimensions))
                .collect(),
        }
    }

    fn quadrants(&self) -> Vec<Self> {
        let quadrant_dims = self.dimensions / 2;

        (0..2)
            .flat_map(|i| {
                (0..2).map(move |j| Self {
                    dimensions: quadrant_dims,
                    robots: self
                        .robots
                        .iter()
                        .filter(|robot| {
                            robot.position.x >= (i * quadrant_dims.x) + i
                                && robot.position.x < ((i + 1) * quadrant_dims.x) + i
                                && robot.position.y >= (j * quadrant_dims.y) + j
                                && robot.position.y < ((j + 1) * quadrant_dims.y) + j
                        })
                        .copied()
                        .collect(),
                })
            })
            .collect()
    }

    fn robot_count(&self) -> usize {
        self.robots.len()
    }

    fn robot_at(&self, position: Vec2i) -> Option<&Robot> {
        self.robots.iter().find(|robot| robot.position == position)
    }

    /// Leftover of failed attempt to assumed peak would be in the middle of the top row
    fn _top_centered_robot(&self) -> Option<Robot> {
        match self.robot_at(Vec2i::new(self.dimensions.x / 2, 0)) {
            Some(&robot) => {
                // return Some(robot);
                if ((0..self.dimensions.x)
                    .filter_map(|x| self.robot_at(Vec2i::new(x, 0)))
                    .count())
                    == 1
                {
                    return Some(robot);
                }
                None
            }
            None => None,
        }
    }

    /// Searches for top robot of this shape:
    ///
    ///   #
    ///  ###
    /// #####
    ///
    /// Determined this shape after first searching for this shape (by guess) and then observing
    /// those results to determine the filled shape above.
    ///
    ///   #
    ///  # #
    /// #   #
    fn peak_robot(&self) -> Option<Robot> {
        self.robots
            .iter()
            .find(|robot| {
                self.robot_at(robot.position + Vec2i::new(-1, 1)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(1, 1)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(-2, 2)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(2, 2)).is_some()
                    // These were added once I manually observed the tree using above criteria
                    && self.robot_at(robot.position + Vec2i::new(0, 1)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(-1, 2)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(0, 2)).is_some()
                    && self.robot_at(robot.position + Vec2i::new(1, 2)).is_some()
            })
            .cloned()
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        (0..self.dimensions.y).for_each(|y| {
            (0..self.dimensions.x).for_each(|x| match self.robot_at(Vec2i::new(x, y)) {
                None => output.push('.'),
                Some(_) => output.push('#'),
            });
            output.push_str("\n");
        });

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
struct Problem {
    area: Area,
}

impl Problem {
    fn from_string(width: u32, height: u32, string: &str) -> Self {
        Self {
            area: Area::from_string(width, height, string),
        }
    }

    fn part_1(&self) -> usize {
        self.area
            .elapse_time(100)
            .quadrants()
            .iter()
            .map(|quadrant| quadrant.robot_count())
            .reduce(|acc, count| acc * count)
            .unwrap()
    }

    /// repeat interval = 10403, so manually seeking not really doable
    fn part_2(&self) -> u32 {
        let mut area = self.area.clone();
        (1..area.robots[0].repeat_interval(area.dimensions))
            .find(|_| {
                area = area.elapse_time(1);
                area.peak_robot().is_some()
            })
            .unwrap()
    }
}

fn main() {
    let problem = Problem::from_string(
        101,
        103,
        fs::read_to_string("input/day14.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 222901875
    println!("Part 2: {}", problem.part_2()); // Attempts: 6243
    println!("Christmas tree:\n{}", problem.area.elapse_time(6243));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(12, Problem::from_string(11, 7, SAMPLE).part_1());
    }
}
