use std::fs;

#[derive(Debug)]
struct CubeGame {
    id: u32,
    sets: Vec<CubeSet>,
}

impl CubeGame {
    pub fn superset(&self) -> CubeSet {
        let mut superset = CubeSet { red: 0, green: 0, blue: 0 };

        self.sets.iter().for_each(|set| {
            if set.red > superset.red {
                superset.red = set.red;
            }
            if set.green > superset.green {
                superset.green = set.green;
            }
            if set.blue > superset.blue {
                superset.blue = set.blue;
            }
        });

        superset
    }
}

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

#[derive(Debug)]
struct State {
    games: Vec<CubeGame>,
}

impl State {
    pub fn from_string(contents: String) -> Self
    {
        Self {
            games: contents.trim().
                lines()
                .map(|line| {
                    let (col_game, col_sets) = line.split_once(": ").unwrap();

                    CubeGame {
                        id: col_game.to_string()[5..].parse::<u32>().unwrap(),
                        sets: col_sets.split("; ")
                            .map(|col_set| {
                                let mut set = CubeSet { red: 0, green: 0, blue: 0 };

                                col_set.split(", ")
                                    .for_each(|col_color| {
                                        let (number, color) = col_color.split_once(" ").unwrap();

                                        match color {
                                            "red" => set.red = number.parse::<u32>().unwrap(),
                                            "green" => set.green = number.parse::<u32>().unwrap(),
                                            _ => set.blue = number.parse::<u32>().unwrap(),
                                        }
                                    });

                                set
                            })
                            .collect(),
                    }
                })
                .collect()
        }
    }

    pub fn sum_possible_game_ids(&self, check_set: CubeSet) -> u32
    {
        self.games.iter()
            .filter(|game| {
                game.sets.iter()
                    .filter(|set| {
                        set.red <= check_set.red &&
                            set.green <= check_set.green &&
                            set.blue <= check_set.blue
                    })
                    .count() == game.sets.iter().count()
            })
            .map(|game| game.id)
            .sum()
    }

    pub fn sum_game_powers(&self) -> u32
    {
        self.games.iter()
            .map(|game| game.superset().power())
            .sum()
    }
}

fn main() {
    let state = State::from_string(fs::read_to_string("inputs/day2.txt").expect("Failed to read input"));

    println!("Part 1: {}", state.sum_possible_game_ids(CubeSet { red: 12, green: 13, blue: 14 }));
    println!("Part 2: {}", state.sum_game_powers());
}