use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Problem {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Problem {
    fn from_string(string: &str) -> Self {
        let (top, bottom) = string.split_once("\n\n").expect("Invalid input");

        Self {
            rules: top
                .lines()
                .map(|line| line.split_once("|").expect("Invalid rule line"))
                .fold(HashMap::new(), |mut rules, (page_left, page_right)| {
                    rules
                        .entry(page_left.parse().unwrap())
                        .or_insert(vec![])
                        .push(page_right.parse().unwrap());

                    rules
                }),
            updates: bottom
                .lines()
                .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn valid_updates(&self) -> Vec<&Vec<u32>> {
        self.filter_updates(true)
    }

    fn invalid_updates(&self) -> Vec<&Vec<u32>> {
        self.filter_updates(false)
    }

    fn filter_updates(&self, keep_valid: bool) -> Vec<&Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| {
                let invalid_page = update
                    .iter()
                    .enumerate()
                    // For each page in the update search for incorrect preceding pages
                    .find(|(update_page_idx, update_page)| {
                        match self.rules.get(update_page) {
                            // No ordering rule found, so this page is correct
                            None => false,
                            // Ordering rules found for given page
                            Some(invalid_pages) => invalid_pages
                                .iter()
                                .find(|invalid_page| {
                                    // Check preceding pages for invalid pages
                                    update[0..*update_page_idx].contains(invalid_page)
                                })
                                .is_some(),
                        }
                    });

                // None indicates that no invalidly ordered pages were found
                (invalid_page.is_none() && keep_valid) || (invalid_page.is_some() && !keep_valid)
            })
            .collect()
    }

    fn part_1(&self) -> u32 {
        self.valid_updates()
            .iter()
            .map(|update| update[(update.len() - 1) / 2]) // Take the middle page
            .sum()
    }

    fn part_2(&self) -> u32 {
        self.invalid_updates()
            .iter()
            .map(|&update| {
                let mut sorted_update = update.clone();

                sorted_update.sort_by(|left, right| match self.rules.get(left) {
                    None => Ordering::Equal,
                    Some(disallowed_preceding) => match disallowed_preceding.contains(right) {
                        true => Ordering::Less,
                        false => Ordering::Equal,
                    },
                });

                sorted_update
            })
            .map(|update| update[(update.len() - 1) / 2]) // Take the middle page
            .sum()
    }
}

fn main() {
    let problem = Problem::from_string(
        fs::read_to_string("input/day5.txt")
            .expect("Failed to read input")
            .as_str(),
    );

    println!("Part 1: {}", problem.part_1()); // Attempts: 5948
    println!("Part 2: {}", problem.part_2()); // Attempts: 3062
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53\n\
                          97|13\n\
                          97|61\n\
                          97|47\n\
                          75|29\n\
                          61|13\n\
                          75|53\n\
                          29|13\n\
                          97|29\n\
                          53|29\n\
                          61|53\n\
                          97|53\n\
                          61|29\n\
                          47|13\n\
                          75|47\n\
                          97|75\n\
                          47|61\n\
                          75|61\n\
                          47|29\n\
                          75|13\n\
                          53|13\n\
                          \n\
                          75,47,61,53,29\n\
                          97,61,53,29,13\n\
                          75,29,13\n\
                          75,97,47,61,53\n\
                          61,13,29\n\
                          97,13,75,29,47";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(143, Problem::from_string(SAMPLE).part_1());
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(123, Problem::from_string(SAMPLE).part_2());
    }
}
