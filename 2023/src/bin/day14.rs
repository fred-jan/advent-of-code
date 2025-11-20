use std::fmt::Debug;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Platform {
    rows: Vec<String>,
}

impl Platform {
    pub fn load(contents: String) -> Self {
        Self {
            rows: contents
                .trim()
                .lines()
                .map(|line| line.to_string())
                .collect(),
        }
    }

    pub fn rotate_ccw(&self) -> Self {
        Self {
            rows: (0..self.rows[0].len())
                .rev()
                .map(|col| {
                    self.rows
                        .iter()
                        .map(|line| line[col..col + 1].to_string())
                        .collect::<String>()
                })
                .collect(),
        }
    }

    pub fn rotate_cw(&self) -> Self {
        Self {
            rows: (0..self.rows[0].len())
                .map(|col| {
                    self.rows
                        .iter()
                        .map(|line| line[col..col + 1].to_string())
                        .rev()
                        .collect::<String>()
                })
                .collect(),
        }
    }

    pub fn move_boulders_left(&self) -> Self {
        Self {
            rows: self
                .rows
                .iter()
                .map(|row| {
                    row.chars()
                        .enumerate()
                        .fold(String::new(), |mut acc, (i, char)| {
                            if char == '#' {
                                // push all occurrences of rocks(#) unaltered
                                acc.push(char);
                            }
                            // First boulder(O) or open space(.), or one preceded by a rock (#)
                            else if i == 0 || &row[i.saturating_sub(1)..i] == "#" {
                                let mut captured = row[i..]
                                    .chars()
                                    .take_while(|&cap_char| cap_char != '#')
                                    .collect::<Vec<char>>();
                                captured.sort_by(|a, b| b.cmp(a));
                                acc.push_str(&captured.iter().collect::<String>());
                            }

                            acc
                        })
                })
                .collect(),
        }
    }

    pub fn slide_north(&self) -> Self {
        self.rotate_ccw().move_boulders_left().rotate_cw()
    }

    pub fn slide_cycle(&self, times: usize) -> Self {
        let mut current = self.clone().rotate_ccw();
        let mut history = vec![];

        for _ in 0..times {
            history.push(current.clone());

            // 4 x (move boulders left -> rotate clockwise)
            current = (0..4).fold(current, |acc, _| acc.move_boulders_left().rotate_cw());

            // Detect cycles
            if let Some(cycle_pos) = history.iter().position(|previous| previous == &current) {
                let cycle_length = history.len() - cycle_pos;
                let cycle_range = times - cycle_pos;
                let cycle_remainder = cycle_range % cycle_length;

                current = history[cycle_pos + cycle_remainder].clone();
                break;
            }
        }

        current.rotate_cw()
    }

    pub fn total_load_north(&self) -> usize {
        self.rows
            .iter()
            .zip((0..self.rows.len() + 1).rev())
            .fold(0, |acc, (row, weight)| {
                acc + row.chars().filter(|&char| char == 'O').count() * weight
            })
    }
}

fn main() {
    let platform = Platform::load(fs::read_to_string("inputs/day14.txt").unwrap());

    // 113424
    println!("Part 1: {}", platform.slide_north().total_load_north());
    // 96003
    println!(
        "Part 2: {}",
        platform.slide_cycle(1_000_000_000).total_load_north()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        let platform = Platform::load(fs::read_to_string("samples/day14.txt").unwrap());

        assert_eq!(
            Platform::load(
                r#"
.....#....
....#...O#
.....##...
...#......
.....OOO#.
.O#...O#.#
....O#...O
......OOOO
#....###.O
#.OOO#..OO
        "#
                .to_string()
            )
            .total_load_north(),
            64
        );

        // Test full rotations
        assert_eq!(
            (0..4).fold(platform.clone(), |acc, _| acc.rotate_ccw()),
            platform
        );
        assert_eq!(
            (0..4).fold(platform.clone(), |acc, _| acc.rotate_cw()),
            platform
        );

        // Test reversibility of rotation
        assert_eq!(platform.rotate_cw().rotate_ccw(), platform);
        assert_eq!(platform.rotate_ccw().rotate_cw(), platform);

        // Test a specific rotation
        assert_eq!(
            platform.rotate_cw(),
            Platform::load(
                r#"
##..O.O.OO
O....OO...
O..O#...O.
......#.O.
......O.#.
##.#O..#.#
.#.O...#..
.#O.#O....
.....#....
...O#.O.#.
"#
                .to_string()
            )
        );

        // Test given cycles
        assert_eq!(
            platform.slide_cycle(1),
            Platform::load(
                r#"
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#
                .to_string()
            )
        );

        assert_eq!(
            platform.slide_cycle(2),
            Platform::load(
                r#"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
            "#
                .to_string()
            )
        );

        assert_eq!(
            platform.slide_cycle(3),
            Platform::load(
                r#"
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"#
                .to_string()
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Platform::load(fs::read_to_string("samples/day14.txt").unwrap())
                .slide_north()
                .total_load_north(),
            136
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Platform::load(fs::read_to_string("samples/day14.txt").unwrap())
                .slide_cycle(1_000_000_000)
                .total_load_north(),
            64
        );
    }
}
