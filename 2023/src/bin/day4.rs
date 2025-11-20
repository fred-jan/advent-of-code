use std::cmp::max;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct ScratchCard {
    card_number: u32,
    win_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl ScratchCard {
    fn matches(&self) -> Vec<&u32> {
        self.card_numbers
            .iter()
            .filter(|number| self.win_numbers.contains(number))
            .collect()
    }

    fn won_card_numbers(&self) -> Vec<u32> {
        (self.card_number + 1..self.card_number + 1 + self.matches().len() as u32).collect()
    }

    fn points(&self) -> u32 {
        self.matches().iter().fold(0, |acc, _| max(1, acc * 2))
    }
}

#[derive(Debug)]
struct Puzzle {
    cards: Vec<ScratchCard>,
}

impl Puzzle {
    pub fn card_by_number(&self, card_number: u32) -> Option<&ScratchCard> {
        self.cards
            .iter()
            .find(|card| card.card_number == card_number)
    }

    pub fn count_won_cards(&self, card: &ScratchCard, counts: &mut HashMap<u32, u32>) -> u32 {
        match counts.get(&card.card_number) {
            Some(count) => count.clone(),
            None => {
                let won_card_numbers = card.won_card_numbers();

                let count = won_card_numbers.len() as u32
                    + card
                        .won_card_numbers()
                        .iter()
                        .map(|&card_number| match self.card_by_number(card_number) {
                            Some(card) => self.count_won_cards(card, counts),
                            None => 0,
                        })
                        .sum::<u32>();

                counts.insert(card.card_number, count);

                count
            }
        }
    }

    fn load(contents: String) -> Self {
        Self {
            cards: contents
                .lines()
                .map(|line| line.split_once(":").unwrap())
                .map(|(lft, rgt)| {
                    let (winning_numbers, card_numbers) = rgt.split_once(" | ").unwrap();

                    ScratchCard {
                        card_number: lft
                            .to_string()
                            .split_ascii_whitespace()
                            .last()
                            .unwrap()
                            .parse::<u32>()
                            .unwrap(),
                        win_numbers: winning_numbers
                            .to_string()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<u32>().unwrap())
                            .collect(),
                        card_numbers: card_numbers
                            .to_string()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<u32>().unwrap())
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    fn pt1(&self) -> u32 {
        self.cards.iter().map(|card| card.points()).sum()
    }

    fn pt2(&self) -> u32 {
        let mut counts = HashMap::new();

        self.cards.len() as u32
            + self
                .cards
                .iter()
                .map(|card| self.count_won_cards(card, &mut counts))
                .sum::<u32>()
    }
}

fn main() {
    let puzzle = Puzzle::load(fs::read_to_string("inputs/day4.txt").expect("Failed to read input"));
    println!("Part 1: {}", puzzle.pt1());
    println!("Part 2: {}", puzzle.pt2());
}

#[test]
fn test_common() {
    assert_eq!(
        ScratchCard {
            card_number: 1,
            win_numbers: vec![1, 2, 3, 4, 5, 6, 7],
            card_numbers: vec![2, 4, 5, 9]
        }
        .points(),
        4
    );

    assert_eq!(
        ScratchCard {
            card_number: 10,
            win_numbers: vec![1, 2, 3, 4, 5, 6, 7],
            card_numbers: vec![1, 2, 3, 4, 5]
        }
        .won_card_numbers(),
        vec![11, 12, 13, 14, 15]
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        Puzzle::load(fs::read_to_string("samples/day4.txt").expect("Failed to read input")).pt1(),
        13
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Puzzle::load(fs::read_to_string("samples/day4.txt").expect("Failed to read input")).pt2(),
        30
    );
}
