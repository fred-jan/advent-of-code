use std::cell::Cell;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Galaxy {
    number: u32,
    x: Cell<usize>,
    y: Cell<usize>,
}

impl Galaxy {
    pub fn shortest_path(&self, to: &Self) -> usize {
        self.x.get().abs_diff(to.x.get()) + self.y.get().abs_diff(to.y.get())
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn load(contents: String) -> Self {
        let mut galaxy_count: u32 = 0;

        Self {
            galaxies: contents
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, char)| {
                            if char != '#' {
                                return None;
                            }

                            galaxy_count += 1;

                            Some(Galaxy {
                                number: galaxy_count,
                                x: Cell::new(x),
                                y: Cell::new(y),
                            })
                        })
                        .collect::<Vec<Galaxy>>()
                })
                .collect(),
            width: contents.find("\n").unwrap(),
            height: contents.lines().count() - 1,
        }
    }

    pub fn get_galaxy(&self, number: u32) -> &Galaxy {
        self.galaxies
            .iter()
            .find(|galaxy| galaxy.number == number)
            .unwrap()
    }

    pub fn expand(&self, times: usize) {
        (0..self.width).rev().for_each(|x| {
            if self
                .galaxies
                .iter()
                .filter(|galaxy| galaxy.x.get() == x)
                .count()
                == 0
            {
                self.galaxies
                    .iter()
                    .filter(|galaxy| galaxy.x.get() > x)
                    .for_each(|galaxy| galaxy.x.set(galaxy.x.get() + times - 1));
            }
        });

        (0..self.height).rev().for_each(|y| {
            if self
                .galaxies
                .iter()
                .filter(|galaxy| galaxy.y.get() == y)
                .count()
                == 0
            {
                self.galaxies
                    .iter()
                    .filter(|galaxy| galaxy.y.get() > y)
                    .for_each(|galaxy| galaxy.y.set(galaxy.y.get() + times - 1));
            }
        });
    }

    pub fn sum_shortest_paths(&self) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, galaxy)| {
                self.galaxies[i + 1..]
                    .iter()
                    .map(|other| galaxy.shortest_path(other))
            })
            .sum()
    }
}

fn main() {
    let universe1 = Universe::load(fs::read_to_string("inputs/day11.txt").unwrap());
    universe1.expand(2);

    // 10313550
    println!("Part 1: {}", universe1.sum_shortest_paths());

    let universe2 = Universe::load(fs::read_to_string("inputs/day11.txt").unwrap());
    universe2.expand(1000000);

    // 611998089572
    println!("Part 2: {}", universe2.sum_shortest_paths());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let universe = Universe::load(fs::read_to_string("samples/day11.txt").unwrap());
        universe.expand(2);

        assert_eq!(
            universe.get_galaxy(1).shortest_path(universe.get_galaxy(7)),
            15
        );
        assert_eq!(
            universe.get_galaxy(3).shortest_path(universe.get_galaxy(6)),
            17
        );
        assert_eq!(
            universe.get_galaxy(8).shortest_path(universe.get_galaxy(9)),
            5
        );
        assert_eq!(universe.sum_shortest_paths(), 374);
    }

    #[test]
    fn test_part2() {
        let universe1 = Universe::load(fs::read_to_string("samples/day11.txt").unwrap());
        universe1.expand(10);
        assert_eq!(universe1.sum_shortest_paths(), 1030);

        let universe2 = Universe::load(fs::read_to_string("samples/day11.txt").unwrap());
        universe2.expand(100);
        assert_eq!(universe2.sum_shortest_paths(), 8410);
    }
}
