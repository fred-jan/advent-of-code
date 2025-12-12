use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    str::FromStr,
    vec,
};

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
        self.schematic.count_paths(
            *self.schematic.devices.get("you").expect("Expect you node"),
            *self.schematic.devices.get("out").expect("Expect out node"),
            vec![],
            HashSet::new(),
            &mut HashMap::new(),
        )
    }

    pub fn part_2(&self) -> usize {
        self.schematic.count_paths(
            *self.schematic.devices.get("svr").expect("Expect svr node"),
            *self.schematic.devices.get("out").expect("Expect out node"),
            vec![],
            vec![
                *self.schematic.devices.get("dac").expect("Expect dac node"),
                *self.schematic.devices.get("fft").expect("Expect fft node"),
            ]
            .into_iter()
            .collect(),
            &mut HashMap::new(),
        )
    }
}

#[derive(Debug, Default)]
struct Schematic {
    devices: HashMap<String, usize>,
    device_labels: Vec<String>, // for debugging
    device_count: usize,
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
    // For debugging purposes
    fn _path_str(&self, path: &[usize]) -> String {
        path.iter()
            .map(|device_idx| self.device_labels[*device_idx].clone())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn count_paths(
        &self,
        from: usize,
        to: usize,
        path: Vec<usize>,
        to_visit: HashSet<usize>,
        memory: &mut HashMap<(usize, Vec<usize>), usize>,
    ) -> usize {
        if path.contains(&from) {
            println!("Loop detected");
            return 0;
        }

        // println!(
        //     "Current: {}, Path: {}",
        //     self.device_labels[from],
        //     self.path_str(&path)
        // );

        let mut path = path.clone();
        path.push(from);

        if let Some(memorized_path_count) = memory.get(&(from, to_visit.iter().copied().collect()))
        {
            // println!(
            //     "Memorized paths for {} with to visit={}",
            //     self.device_labels[from],
            //     self._path_str(&to_visit.iter().copied().collect::<Vec<usize>>())
            // );
            return *memorized_path_count;
        }

        let mut to_visit = to_visit.clone();
        to_visit.remove(&from);

        if from == to {
            // Only return the path if we visited all nodes, otherwise return no paths
            return if to_visit.is_empty() { 1 } else { 0 };
        }

        let path_count = self.mappings[&from]
            .iter()
            .map(|output| self.count_paths(*output, to, path.clone(), to_visit.clone(), memory))
            .sum();

        // println!(
        //     "Memorizing paths for {} with to visit={}",
        //     self.device_labels[from],
        //     self._path_str(&to_visit.iter().copied().collect::<Vec<usize>>())
        // );
        memory.insert((from, to_visit.iter().copied().collect()), path_count);

        path_count
    }

    fn register_device(&mut self, label: String) -> usize {
        if let Some(device_idx) = self.devices.get(&label) {
            return *device_idx;
        }

        let device_idx = self.device_count;

        self.device_labels.push(label.clone());
        self.devices.insert(label, device_idx);
        self.device_count += 1;

        device_idx
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day11.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1()); // 428
    println!("Part 2: {}", problem.part_2()); // 331468292364745

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

    const SAMPLE2: &str = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(5, SAMPLE.parse::<Problem>().unwrap().part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(2, SAMPLE2.parse::<Problem>().unwrap().part_2());
    }
}
