use std::{collections::HashMap, error::Error, fs, str::FromStr, vec};

struct Problem {
    schematic: Schematic,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            schematic: s.trim().parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self) -> usize {
        self.schematic.find_paths(self.schematic.main_input, vec![]).len()
    }
}

#[derive(Debug, Default)]
struct Schematic {
    devices: HashMap<String, usize>,
    device_count: usize,
    main_input: usize,
    main_output: usize,
    mappings: HashMap<usize, Vec<usize>>,
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instance = Self::default();

        s.lines().for_each(|l| {
            let (input_str, outputs_str) = l.split_once(": ").expect("Expected semicolon");
            let input_idx = instance.register_device(input_str.to_string());
            let output_idxs = outputs_str
                .split_whitespace()
                .map(|output_label| instance.register_device(output_label.to_string()))
                .collect();
            instance.mappings.insert(input_idx, output_idxs);
        });

        Ok(instance)
    }
}

impl Schematic {
    fn find_paths(&self, current: usize, current_path: Vec<usize>) -> Vec<Vec<usize>> {
        let mut current_path = current_path.clone();
        current_path.push(current);

        if current == self.main_output {
            return vec![current_path];
        }

        self.mappings[&current]
            .iter()
            .flat_map(|output| self.find_paths(*output, current_path.clone()))
            .collect()
    }

    fn register_device(&mut self, label: String) -> usize {
        if let Some(device_idx) = self.devices.get(&label) {
            return *device_idx;
        }

        let device_idx = self.device_count;

        if label == "you" {
            self.main_input = device_idx;
        }

        if label == "out" {
            self.main_output = device_idx;
        }

        self.devices.insert(label, device_idx);
        self.device_count += 1;

        device_idx
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day11.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // 428

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(5, SAMPLE.parse::<Problem>().unwrap().part_1());
    }
}
