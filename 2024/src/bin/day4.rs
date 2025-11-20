use std::fs;

struct Problem {
    width: usize,
    chars: Vec<char>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        Self {
            width: string.find("\n").unwrap(),
            chars: string.lines().flat_map(|line| line.chars()).collect(),
        }
    }

    fn char_at(&self, x: usize, y: usize) -> Option<&char> {
        if x >= self.width {
            return None;
        }

        self.chars.get(x + y * self.width)
    }

    fn directions(&self) -> [(i32, i32); 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }

    fn word_count(&self, word: String) -> usize {
        self.chars
            .iter()
            .enumerate()
            // Minimal optimization: only search for chars matching first letter of the word
            .filter(|(_, &char)| char == word.chars().nth(0).unwrap())
            .map(|(index, _)| {
                let x = index % self.width;
                let y = index / self.width;

                self.directions()
                    .iter()
                    .filter(|(dx, dy)| {
                        (0..word.len())
                            .filter_map(|i| {
                                let x2 = x as i32 + (i as i32 * dx);
                                let y2 = y as i32 + (i as i32 * dy);

                                if x2 < 0 || y2 < 0 {
                                    return None;
                                }

                                self.char_at(x2 as usize, y2 as usize)
                            })
                            .collect::<String>()
                            == word
                    })
                    .count()
            })
            .sum()
    }

    fn part_1(&self) -> usize {
        self.word_count("XMAS".to_string())
    }

    /// Lazy implementation for part 2
    fn part_2(&self) -> usize {
        self.chars
            .iter()
            .enumerate()
            .filter(|(_, &char)| char == 'A') // only look at tiles containing an A
            .map(|(index, _)| (index % self.width, index / self.width)) // index to coordinates
            .filter(|(x, y)| *x > 0 && *y > 0) // ignore first row and column (prevents overflow)
            .filter(|(x, y)| {
                let seq = vec![
                    self.char_at(x - 1, y - 1),
                    self.char_at(x + 1, y - 1),
                    self.char_at(x - 1, y + 1),
                    self.char_at(x + 1, y + 1),
                ]
                .iter()
                .filter_map(|&char| char) // Remove nones
                .collect::<String>();

                seq == "MMSS" || seq == "SSMM" || seq == "SMSM" || seq == "MSMS"
            })
            .count()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day4.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 2545
    println!("Part 2: {}", problem.part_2()); // Attempts: 1886
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM\n\
                          MSAMXMSMSA\n\
                          AMXSXMAAMM\n\
                          MSAMASMSMX\n\
                          XMASAMXAMM\n\
                          XXAMMXXAMA\n\
                          SMSMSASXSS\n\
                          SAXAMASAAA\n\
                          MAMMMXMMMM\n\
                          MXMXAXMASX";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(18, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(9, Problem::from_string(SAMPLE).part_2());
    }

    #[test]
    fn test_char_at() {
        let problem = Problem::from_string(
            "123\n\
            456\n\
            789",
        );

        assert_eq!(Some(&'1'), problem.char_at(0, 0));
        assert_eq!(Some(&'2'), problem.char_at(1, 0));
        assert_eq!(Some(&'3'), problem.char_at(2, 0));
        assert_eq!(Some(&'4'), problem.char_at(0, 1));
        assert_eq!(Some(&'5'), problem.char_at(1, 1));
        assert_eq!(Some(&'6'), problem.char_at(2, 1));
        assert_eq!(Some(&'7'), problem.char_at(0, 2));
        assert_eq!(Some(&'8'), problem.char_at(1, 2));
        assert_eq!(Some(&'9'), problem.char_at(2, 2));
    }
}
