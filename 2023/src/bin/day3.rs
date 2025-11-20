use std::fs;

// todo: refactor using part 2 code
fn pt1_sum(contents: String) -> u32 {
    contents.lines()
        .enumerate()
        .map(|(i_line, line)| {
            line.chars()
                .enumerate()
                // Only digits with no preceding digit (start of a number)
                .filter(|(i_char, char)| {
                    char.is_digit(10) &&
                        (i_char == &0usize || !line.as_bytes()[i_char - 1].is_ascii_digit())
                })
                // Transform to tuples of numbers with their starting index
                .map(|(i_char, _)| {
                    (i_char, line[i_char..].chars()
                        .take_while(|char| char.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap())
                })
                // Only keep numbers without neighbouring symbols
                .filter(|(i_number, number)| {
                    contents.lines()
                        .enumerate()
                        // Previous line until next line
                        .filter(|(i, _)| (i_line.saturating_sub(1)..i_line.checked_add(2).unwrap_or(i_line)).contains(i))
                        // Only keep lines that have one or more symbols in surrounding columns
                        .filter(|(_, line)| {
                            line.chars()
                                .skip(i_number.clone().saturating_sub(1)) // col start
                                .take(number.to_string().len() + 2) // col end
                                .filter(|char| !char.is_digit(10) && char.clone() != '.')
                                .count() > 0
                        })
                        // Only keep numbers with neighbouring symbols
                        .count() > 0
                })
                .map(|(_, number)| number)
                .sum::<u32>()
        })
        .sum()
}


#[derive(Debug)]
struct Schematic {
    parts: Vec<Part>,
}

impl Schematic {
    fn load(contents: String) -> Self {
        Self {
            parts: contents.lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, char)| char.clone() != '.') // todo: compare without clone?
                        .filter(|(x, char)| {
                            // Only symbols or digits without preceding digits (start of number sequence)
                            !char.is_digit(10) || x.clone() == 0usize || !line.as_bytes()[x.saturating_sub(1)].is_ascii_digit()
                        })
                        .fold(vec![], |mut parts: Vec<Part>, (x, char)| {
                            if char.is_digit(10) {
                                parts.push(Part {
                                    x,
                                    y,
                                    kind: PartKind::Number(
                                        line[x..].chars()
                                            .take_while(|char| char.is_digit(10))
                                            .collect::<String>()
                                            .parse::<u16>()
                                            .unwrap()
                                    ),
                                })
                            } else {
                                parts.push(Part { x, y, kind: PartKind::Symbol(char) });
                            }

                            parts
                        })
                })
                .flatten()
                .collect()
        }
    }

    fn neighbour_parts(&self, origin_part: &Part) -> Vec<&Part> {
        self.parts.iter()
            .filter(|&part| !part.eq(origin_part) && origin_part.is_neighbour(part))
            .collect()
    }

    fn gear_ratio_sum(&self) -> u32 {
        self.parts.iter()
            .filter(|part| match part.kind {
                PartKind::Symbol('*') => true,
                _ => false
            })
            .map(|gear_part| {
                let neighbour_numbers: Vec<u32> = self.neighbour_parts(gear_part)
                    .iter()
                    .filter_map(|neighbour_part| {
                        match neighbour_part.kind {
                            PartKind::Number(num) => Some(num as u32),
                            _ => None
                        }
                    })
                    .collect();

                if neighbour_numbers.len() != 2 {
                    return 0;
                }

                neighbour_numbers.iter().fold(1, |acc, num| acc * num)
            })
            .sum()
    }
}

#[derive(Debug)]
enum PartKind {
    Number(u16),
    Symbol(char),
}

#[derive(Debug)]
struct Part {
    x: usize,
    y: usize,
    kind: PartKind,
}

impl Part {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn x_right(&self) -> usize
    {
        match self.kind {
            PartKind::Number(num) => self.x + num.to_string().len() - 1,
            _ => self.x
        }
    }

    fn is_neighbour(&self, part: &Self) -> bool
    {
        if part.y < self.y.saturating_sub(1) || part.y > self.y + 1 {
            return false;
        }

        if part.x_right() < self.x.saturating_sub(1) || part.x > self.x_right() + 1 {
            return false;
        }
        true
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Failed to read input");

    println!("Part 1: {}", pt1_sum(contents));
    println!("Part 2: {}", Schematic::load(fs::read_to_string("inputs/day3.txt").expect("Failed to read input"))
        .gear_ratio_sum());
}

#[test]
fn test_part1() {
    assert_eq!(
        pt1_sum(fs::read_to_string("samples/day3.txt").expect("Failed to read input")),
        4361
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Schematic::load(fs::read_to_string("samples/day3.txt").expect("Failed to read input"))
            .gear_ratio_sum(),
        467835
    );
}