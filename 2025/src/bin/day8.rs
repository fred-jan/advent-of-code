use std::{collections::HashMap, error::Error, fmt::Display, fs, ops::Sub, str::FromStr, vec};

struct Problem {
    playground: Playground,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            playground: s.trim().parse()?,
        })
    }
}

impl Problem {
    pub fn part_1(&self, n: usize) -> usize {
        self.playground.product_max_3_circuit_sizes(n)
    }

    pub fn part_2(&self) -> usize {
        todo!()
    }
}

struct Playground {
    boxes: Vec<Coordinate>,
}

impl FromStr for Playground {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            boxes: s
                .lines()
                .map(|l| {
                    l.parse::<Coordinate>()
                        .map_err(|_| "Failed to parse coordinate")
                })
                .collect::<Result<Vec<Coordinate>, _>>()?,
        })
    }
}

impl Playground {
    fn product_max_3_circuit_sizes(&self, n_connections: usize) -> usize {
        let mut dists: HashMap<(usize, usize), f32> = HashMap::new();

        for (i, i_loc) in self.boxes.iter().enumerate() {
            for (j, j_loc) in self.boxes.iter().enumerate() {
                if i == j {
                    // Exclude wiring a box with itself
                    continue;
                }

                // Construct key using tuple (<lowest box index>, <highest box index>)
                let k = if i > j { (j, i) } else { (i, j) };

                if dists.contains_key(&k) {
                    continue;
                }

                dists.insert(k, i_loc.euclid_dist(j_loc));
            }
        }

        // Sort the distances from lowest to highest
        let mut sorted_dists: Vec<((usize, usize), f32)> = dists.into_iter().collect();
        sorted_dists.sort_by(|a, b| a.1.total_cmp(&b.1));

        let mut circuits: Vec<Vec<usize>> = vec![];

        // Connect lowest n boxes
        for ((i, j), _dist) in sorted_dists.into_iter().take(n_connections) {
            // println!(
            //     "Connecting {} with {} (dist: {})",
            //     self.boxes[i], self.boxes[j], dist
            // );

            let i_circuit_idx = circuits
                .iter()
                .position(|circuit_boxes| circuit_boxes.contains(&i));
            let j_circuit_idx = circuits
                .iter()
                .position(|circuit_boxes| circuit_boxes.contains(&j));

            match (i_circuit_idx, j_circuit_idx) {
                (Some(idx_i), Some(idx_j)) => {
                    if idx_i == idx_j {
                        // Boxes are already in same circuit, nothing happens
                        continue;
                    }

                    // Boxes in two separate circuits, connect the circuits
                    let mut j_circuit = circuits.swap_remove(idx_j);
                    circuits[idx_i].append(&mut j_circuit);
                }
                (Some(idx_i), None) => circuits[idx_i].push(j),
                (None, Some(idx_j)) => circuits[idx_j].push(i),
                (None, None) => circuits.push(vec![i, j]),
            }
        }

        let mut circuit_sizes: Vec<usize> =
            circuits.into_iter().map(|circuit| circuit.len()).collect();
        circuit_sizes.sort_by(|a, b| b.cmp(a));
        circuit_sizes.iter().take(3).product()
    }
}

#[derive(Clone)]
struct Coordinate {
    x: u64,
    y: u64,
    z: u64,
}

impl Coordinate {
    fn euclid_dist(&self, other: &Self) -> f32 {
        let diff = other.clone() - self.clone();

        (((diff.x.pow(2)) + (diff.y.pow(2)) + (diff.z.pow(2))) as f32).sqrt()
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(",").map(|component| {
            component
                .parse()
                .map_err(|_| "Location component is not an integer")
        });
        Ok(Self {
            x: components.next().ok_or("Missing X component")??,
            y: components.next().ok_or("Missing Y component")??,
            z: components.next().ok_or("Missing Z component")??,
        })
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x.abs_diff(self.x),
            y: rhs.y.abs_diff(self.y),
            z: rhs.z.abs_diff(self.z),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let problem = fs::read_to_string("input/day8.txt")?.parse::<Problem>()?;

    println!("Part 1: {}", problem.part_1(1000)); // Attempts: 8, 57564

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_sample_part_1() {
        assert_eq!(40, SAMPLE.parse::<Problem>().unwrap().part_1(10));
    }
}
