use std::{error::Error, fs, str::FromStr, vec};

struct Problem {
    machines: Vec<Machine>,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            machines: s
                .trim()
                .lines()
                .map(|l| l.parse::<Machine>())
                .collect::<Result<Vec<Machine>, _>>()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.machines.iter().map(|m| m.fewest_presses_sum()).sum()
    }
}

#[derive(Clone, Debug)]
struct Machine {
    lights_goal: Vec<bool>,
    lights_current: Vec<bool>,
    button_wirings: Vec<Vec<usize>>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        let (lights_str, right) = s
            .split_once("] ")
            .ok_or("No lights closing bracket found")?;

        let (wirings_str, _joltages_str) = right
            .rsplit_once(" ")
            .ok_or("Unable to split button wirings from joltages")?;

        Ok(Self {
            lights_goal: lights_str[1..].chars().map(|c| c == '#').collect(),
            lights_current: vec![false; lights_str.len() - 1],
            button_wirings: wirings_str
                .split_whitespace()
                .map(|wiring_str| {
                    wiring_str[1..wiring_str.len() - 1]
                        .split(',')
                        .map(|button_str| {
                            button_str
                                .parse::<usize>()
                                .map_err(|_| "Invalid button number in wiring")
                        })
                        .collect::<Result<Vec<usize>, _>>()
                })
                .collect::<Result<Vec<Vec<usize>>, _>>()?,
        })
    }
}

impl Machine {
    // TODO: optimize (change to permutations, excluding repeated same button)
    fn button_combinations(&self, number: usize, result: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut new_result = vec![];

        let i = match result.first() {
            Some(elem) => elem.len(),
            None => 0,
        };

        // Subset to pick elements from
        let subset = i..self.button_wirings.len();

        if number > self.button_wirings.len() {
            panic!("Combination length larger than choose space");
        }

        if result.is_empty() {
            new_result = subset
                .map(|button| vec![button])
                .collect::<Vec<Vec<usize>>>();
        } else {
            // Forms combinations [0,1], [1,1], [2,1], [3,1], [0,2], [1,2], [2,2], [2,3], [0,3],
            // [1,3], [2,3], [3,3]
            for combination in result.iter() {
                // [0,1], [1,1], [2,1], [3,1]
                // [0,2], [1,2], [2,2], [3,2]
                // [0,3], [1,3], [2,3], [3,3]
                for button in subset.clone() {
                    let mut new_combination = combination.clone();
                    new_combination.push(button);
                    new_result.push(new_combination);
                }
            }
        }

        if i + 1 < number {
            self.button_combinations(number, new_result)
        } else {
            new_result
        }
    }

    fn fewest_presses_sum(&self) -> usize {
        // Optimal approach:
        // Collect buttons that toggle the same light, number these as b_x
        // Obtain an equation for that light in form b_0 + b_1 + ... + b_n = 0|1 (mod 2)
        // Solve the system of equations having an equation for each light
        //
        // Brute force:
        // Try button b_0, b_1, ..., b_n in turn
        // Try button b_0->b_1, b_0->b_2, ... b_0->b_n in turn (permutation 1 with 2 buttons)
        // Try button b_1->b_2, b_1->b_3, ... b_1->b_n in turn (permutation 2 with 2 buttons)

        // let button_count =
        // first try: 0 1, 2, 3, 4, 5 (check lights after each)
        // 2nd round: 0+1, 0+2, 0+3, 0+4, 0+5
        // 3rd round: 1+2, 1+3, 1+4, 1+5

        let mut machine = self.clone();

        // Try combination size of 1 to total number of buttons, in this order
        for combination_size in 1..machine.button_wirings.len() {
            for combination in self.button_combinations(combination_size, vec![]) {
                machine.reset_lights();

                for button in combination {
                    machine.press_button(button);
                }

                if machine.lights_current == machine.lights_goal {
                    return combination_size;
                }
            }
        }

        panic!("No combination found");
    }

    fn press_button(&mut self, n: usize) {
        for toggle_light in self.button_wirings[n].iter() {
            self.lights_current[*toggle_light] = !self.lights_current[*toggle_light];
        }
    }

    fn reset_lights(&mut self) {
        self.lights_current = vec![false; self.lights_current.len()];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day10.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // 526 (too low), 571

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(7, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
