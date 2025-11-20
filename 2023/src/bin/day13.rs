use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Pattern {
    contents: String,
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    pub fn load(contents: String) -> Self {
        Self {
            contents: contents.to_string(),
            rows: contents.lines().map(|line| line.to_string()).collect(),
            cols: (0..contents.lines().nth(0).unwrap().len())
                .map(|col| {
                    contents
                        .lines()
                        .map(|line| line[col..col + 1].to_string())
                        .collect::<String>()
                })
                .collect(),
        }
    }

    pub fn reflection_rows(&self) -> Vec<usize> {
        self.reflection_indices(&self.rows)
            .iter()
            .map(|&index| index + 1)
            .collect()
    }

    pub fn reflection_cols(&self) -> Vec<usize> {
        self.reflection_indices(&self.cols)
            .iter()
            .map(|&index| index + 1)
            .collect()
    }

    // Returns (fixed summary, fixed pattern)
    pub fn fix_smudge(&self) -> (usize, Self) {
        self.contents
            .chars()
            .enumerate()
            .find_map(|(i, char)| match char {
                '.' | '#' => {
                    let mut fixed_contents = self.contents.clone();
                    fixed_contents.replace_range(i..i + 1, if char == '.' { "#" } else { "." });
                    let fixed_pattern = Self::load(fixed_contents);

                    let old_reflection_rows = self.reflection_rows();
                    let new_reflection_rows = fixed_pattern.reflection_rows();

                    if !new_reflection_rows.is_empty() && new_reflection_rows != old_reflection_rows
                    {
                        return Some((
                            new_reflection_rows
                                .iter()
                                .find(|rows| !old_reflection_rows.contains(rows))
                                .unwrap()
                                * 100,
                            fixed_pattern,
                        ));
                    }

                    let old_reflection_cols = self.reflection_cols();
                    let new_reflection_cols = fixed_pattern.reflection_cols();

                    if !new_reflection_cols.is_empty() && new_reflection_cols != old_reflection_cols
                    {
                        return Some((
                            *new_reflection_cols
                                .iter()
                                .find(|cols| !old_reflection_cols.contains(cols))
                                .unwrap(),
                            fixed_pattern,
                        ));
                    }

                    None
                }
                _ => None,
            })
            .unwrap()
    }

    pub fn summary(&self) -> usize {
        self.reflection_rows().first().unwrap_or(&0usize) * 100
            + self.reflection_cols().first().unwrap_or(&0usize)
    }

    fn reflection_indices(&self, lines: &Vec<String>) -> Vec<usize> {
        lines[..lines.len() - 1] // All except the last (which has no following lines to compare with)
            .iter()
            .enumerate()
            .filter_map(|(index, _)| {
                // Basically cuts the lines at current index and flips the order of preceding lines. Then zips this with
                // the following lines and iterates through the zipped lists element-wise to check if they are equal:
                //
                // ABCDEFGH --cut-in-half-> ABCD,EFGH --flip left--> DCBA,EFGH --compare-index-wise-> D vs E, C vs F, ..
                let reflected_count = lines[0..index + 1]
                    .iter()
                    .rev()
                    .zip(&lines[index + 1..])
                    .filter(|(left, right)| left == right)
                    .count();
                let lines_left = index + 1;
                let lines_right = lines.len() - (index + 1);

                // Preceding (left) or following (right) lines exhausted -> perfect reflection
                if reflected_count == lines_left || reflected_count == lines_right {
                    return Some(index);
                }

                None
            })
            .collect()
    }
}

#[derive(Debug)]
struct Puzzle {
    patterns: Vec<Pattern>,
}

impl Puzzle {
    pub fn load(contents: String) -> Self {
        Self {
            patterns: contents
                .split("\n\n")
                .map(|section| Pattern::load(section.to_string()))
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.patterns
            .iter()
            .fold(0, |acc, pattern| acc + pattern.summary())
    }

    pub fn part2(&self) -> usize {
        self.patterns
            .iter()
            .fold(0, |acc, pattern| acc + pattern.fix_smudge().0)
    }
}

fn main() {
    // 29846
    println!(
        "Part 1: {}",
        Puzzle::load(fs::read_to_string("inputs/day13.txt").unwrap()).part1()
    );
    // 25401
    println!(
        "Part 2: {}",
        Puzzle::load(fs::read_to_string("inputs/day13.txt").unwrap()).part2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap());

        assert_eq!(puzzle.patterns[0].reflection_cols(), vec![5]);
        assert_eq!(puzzle.patterns[1].reflection_rows(), vec![4]);

        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13b.txt").unwrap());

        assert_eq!(puzzle.patterns[0].reflection_rows(), vec![]);
        assert_eq!(puzzle.patterns[0].reflection_cols(), vec![3]);

        assert_eq!(puzzle.patterns[1].reflection_rows(), vec![]);
        assert_eq!(puzzle.patterns[1].reflection_cols(), vec![11]);

        assert_eq!(puzzle.patterns[2].reflection_rows(), vec![]);
        assert_eq!(puzzle.patterns[2].reflection_cols(), vec![1]);

        assert_eq!(puzzle.patterns[3].reflection_cols(), vec![1, 10]); // has multiple reflections
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap()).part1(),
            405
        );
    }

    #[test]
    fn test_part2() {
        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap());

        assert_eq!(puzzle.patterns[0].fix_smudge().1.reflection_rows(), vec![3]);
        assert_eq!(puzzle.patterns[0].fix_smudge().1.reflection_cols(), vec![5]); // still a valid reflection after fix

        assert_eq!(
            puzzle.patterns[1].fix_smudge().1.reflection_rows(),
            vec![1, 4]
        );

        assert_eq!(puzzle.part2(), 400);
    }
}
