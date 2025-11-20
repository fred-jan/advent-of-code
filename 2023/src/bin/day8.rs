use num::integer::lcm;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
}

impl Map {
    pub fn load(contents: String) -> Self {
        let (directions, nodes) = contents.split_once("\n\n").unwrap();

        Self {
            directions: directions
                .chars()
                .map(|char| match char {
                    'L' => Direction::Left,
                    _ => Direction::Right,
                })
                .collect(),
            nodes: nodes
                .lines()
                .map(|line| Node {
                    label: line[0..3].to_string(),
                    left: line[7..10].to_string(),
                    right: line[12..15].to_string(),
                })
                .collect(),
        }
    }

    pub fn node(&self, label: &String) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label.eq(label))
    }

    pub fn next_node(&self, node: &Node, direction: &Direction) -> &Node {
        self.node(node.direction_label(&direction)).unwrap()
    }

    pub fn apply_directions<'a>(&'a self, start: &'a Node) -> &Node {
        self.directions
            .iter()
            .fold(start, |node, direction| self.next_node(node, direction))
    }

    pub fn steps_pt1(&self, node: &Node) -> u64 {
        let mut iterations = 1;
        let mut current = node;

        loop {
            current = self.apply_directions(current);

            if current.label == "ZZZ" {
                break;
            }

            iterations += 1;
        }

        iterations * self.directions.len() as u64
    }

    pub fn part1(&self) -> u64 {
        self.steps_pt1(self.node(&"AAA".to_string()).unwrap())
    }

    pub fn steps_pt2(&self, node: &Node) -> u64 {
        let mut steps = 0;
        let mut current = node;

        loop {
            match self
                .directions
                .iter()
                .enumerate()
                .find_map(|(i, direction)| {
                    current = self.next_node(current, direction);

                    if current.label.ends_with("Z") {
                        return Some(i);
                    }

                    None
                }) {
                None => steps += self.directions.len(),
                Some(i) => {
                    steps += i + 1;
                    break;
                }
            }
        }

        steps as u64
    }

    pub fn part2(&self) -> u64 {
        self.nodes
            .iter()
            .filter(|node| node.label.ends_with("A"))
            .map(|node| self.steps_pt2(node))
            .reduce(|acc, steps| lcm(acc, steps))
            .unwrap()
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    pub fn direction_label(&self, direction: &Direction) -> &String {
        match direction {
            Direction::Left => &self.left,
            _ => &self.right,
        }
    }
}

fn main() {
    println!(
        "Part 1: {}",
        Map::load(fs::read_to_string("inputs/day8.txt").unwrap()).part1()
    );
    println!(
        "Part 2: {}",
        Map::load(fs::read_to_string("inputs/day8.txt").unwrap()).part2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Map::load(fs::read_to_string("samples/day8a.txt").unwrap()).part1(),
            2
        );
        assert_eq!(
            Map::load(fs::read_to_string("samples/day8b.txt").unwrap()).part1(),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Map::load(fs::read_to_string("samples/day8c.txt").unwrap()).part2(),
            6
        );
    }
}
