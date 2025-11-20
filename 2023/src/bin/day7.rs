extern crate core;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Card {
    label: char,
    strength: u32,
}

impl Card {
    pub fn new(label: char) -> Self {
        Self {
            label,
            strength: match label {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '*' => 1, // * = joker
                _ => label.to_digit(10).unwrap(),
            },
        }
    }
}

#[derive(Debug)]
struct RankedHand<'b> {
    identified_hand: &'b IdentifiedHand<'b>,
    rank: u32,
}

impl RankedHand<'_> {
    pub fn winnings(&self) -> u32 {
        self.identified_hand.hand.bid * self.rank
    }
}

#[derive(Debug)]
struct IdentifiedHand<'a> {
    hand: &'a Hand,
    hand_type: HandType,
}

impl IdentifiedHand<'_> {
    pub fn compare(&self, other: &Self) -> Ordering {
        let ord = self.hand_type.cmp(&other.hand_type);

        if ord.is_eq() {
            return zip(&self.hand.cards, &other.hand.cards)
                .find_map(|(a, b)| {
                    let result = a.strength.cmp(&b.strength);

                    match result {
                        Ordering::Equal => None,
                        _ => Some(result),
                    }
                })
                .unwrap_or(Ordering::Equal);
        }

        ord
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn from_string(string: String) -> Self {
        let (col_hand, col_bid) = string.split_once(' ').unwrap();
        Self {
            cards: col_hand
                .chars()
                .map(|card_label| Card::new(card_label))
                .collect(),
            bid: col_bid.parse::<u32>().unwrap(),
        }
    }

    pub fn identify(&self) -> IdentifiedHand {
        let mut label_freqs: HashMap<char, u32> =
            self.cards.iter().fold(HashMap::new(), |mut freqs, card| {
                match freqs.get(&card.label) {
                    Some(count) => freqs.insert(card.label, count + 1),
                    None => freqs.insert(card.label, 1),
                };
                freqs
            });

        let joker_count = label_freqs.get(&'*').unwrap_or(&0u32).clone();
        label_freqs.remove(&'*');

        IdentifiedHand {
            hand: self,
            hand_type: match label_freqs.values().max().unwrap_or(&0) + joker_count {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => {
                    if label_freqs.len() == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                2 => {
                    if label_freqs.values().filter(|&freq| freq >= &2u32).count() == 2 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                _ => HandType::HighCard,
            },
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

struct Puzzle {
    hands: Vec<Hand>,
}

impl Puzzle {
    pub fn load(contents: String) -> Self {
        Self {
            hands: contents
                .lines()
                .map(|line| Hand::from_string(line.to_string()))
                .collect(),
        }
    }

    pub fn load_pt2(contents: String) -> Self {
        Self::load(contents.replace("J", "*"))
    }

    pub fn total_winnings(&self) -> u32 {
        let mut identified_hands = self
            .hands
            .iter()
            .map(|hand| hand.identify())
            .collect::<Vec<IdentifiedHand>>();
        identified_hands.sort_by(|a, b| a.compare(b));
        identified_hands
            .iter()
            .enumerate()
            .map(|(i, identified_hand)| RankedHand {
                identified_hand,
                rank: (i + 1) as u32,
            })
            .map(|ranked_hand| ranked_hand.winnings())
            .sum()
    }
}

fn main() {
    println!(
        "Part 1: {}",
        Puzzle::load(fs::read_to_string("inputs/day7.txt").unwrap()).total_winnings()
    );
    // Attempts: 249591015 -> 250892960 -> 249631254
    println!(
        "Part 2: {}",
        Puzzle::load_pt2(fs::read_to_string("inputs/day7.txt").unwrap()).total_winnings()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranking() {}

    #[test]
    fn test_common() {
        assert_eq!(
            Hand::from_string("KKKKK 123".to_string())
                .identify()
                .hand_type,
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::from_string("KKKKA 123".to_string())
                .identify()
                .hand_type,
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::from_string("KKKAA 123".to_string())
                .identify()
                .hand_type,
            HandType::FullHouse
        );
        assert_eq!(
            Hand::from_string("KKKAQ 123".to_string())
                .identify()
                .hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_string("KTJJT 123".to_string())
                .identify()
                .hand_type,
            HandType::TwoPair
        );
        assert_eq!(
            Hand::from_string("KKAJQ 123".to_string())
                .identify()
                .hand_type,
            HandType::OnePair
        );
        assert_eq!(
            Hand::from_string("KTAJQ 123".to_string())
                .identify()
                .hand_type,
            HandType::HighCard
        );

        // Compare strengths for cards with equal types
        let hand_1 = IdentifiedHand {
            hand: &Hand::from_string("22222 123".to_string()),
            hand_type: HandType::HighCard,
        };
        let hand_2 = IdentifiedHand {
            hand: &Hand::from_string("11111 123".to_string()),
            hand_type: HandType::HighCard,
        };
        let hand_3 = IdentifiedHand {
            hand: &Hand::from_string("33333 123".to_string()),
            hand_type: HandType::HighCard,
        };
        assert_eq!(hand_1.compare(&hand_1), Ordering::Equal);
        assert_eq!(hand_1.compare(&hand_2), Ordering::Greater);
        assert_eq!(hand_1.compare(&hand_3), Ordering::Less);

        // Specific case from sample (2x two pairs -> compare strengths)
        assert_eq!(
            IdentifiedHand {
                hand: &Hand::from_string("KK677 28".to_string()),
                hand_type: HandType::HighCard,
            }
            .compare(&IdentifiedHand {
                hand: &Hand::from_string("KTJJT 220".to_string()),
                hand_type: HandType::HighCard,
            }),
            Ordering::Greater
        );

        assert_eq!(
            IdentifiedHand {
                hand: &Hand::from_string("JJJJJ 666".to_string()),
                hand_type: HandType::FiveOfAKind,
            }
            .compare(&IdentifiedHand {
                hand: &Hand::from_string("99979 459".to_string()),
                hand_type: HandType::FourOfAKind,
            }),
            Ordering::Greater
        );

        assert_eq!(
            IdentifiedHand {
                hand: &Hand::from_string("99959 922".to_string()),
                hand_type: HandType::FourOfAKind,
            }
            .compare(&IdentifiedHand {
                hand: &Hand::from_string("99899 277".to_string()),
                hand_type: HandType::FourOfAKind,
            }),
            Ordering::Greater
        );

        assert_eq!(
            IdentifiedHand {
                hand: &Hand::from_string("99J99 377".to_string()),
                hand_type: HandType::FourOfAKind,
            }
            .compare(&IdentifiedHand {
                hand: &Hand::from_string("99899 288".to_string()),
                hand_type: HandType::FourOfAKind,
            }),
            Ordering::Greater
        );
    }

    #[test]
    fn test_joker() {
        assert_eq!(
            Hand::from_string("*KKKK 123".to_string())
                .identify()
                .hand_type,
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::from_string("1*KKK 123".to_string())
                .identify()
                .hand_type,
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::from_string("11*KK 123".to_string())
                .identify()
                .hand_type,
            HandType::FullHouse
        );
        assert_eq!(
            Hand::from_string("12*KK 123".to_string())
                .identify()
                .hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_string("12**K 123".to_string())
                .identify()
                .hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_string("12*** 123".to_string())
                .identify()
                .hand_type,
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::from_string("1**** 123".to_string())
                .identify()
                .hand_type,
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::from_string("***** 123".to_string())
                .identify()
                .hand_type,
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_part1() {
        let puzzle = Puzzle::load(fs::read_to_string("samples/day7.txt").unwrap());
        assert_eq!(puzzle.total_winnings(), 6440);
    }

    #[test]
    fn test_part2() {
        let puzzle = Puzzle::load_pt2(fs::read_to_string("samples/day7.txt").unwrap());
        assert_eq!(puzzle.total_winnings(), 5905);
    }
}
