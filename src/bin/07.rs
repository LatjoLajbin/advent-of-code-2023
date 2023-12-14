use core::hash::Hash;
use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Num(u32) = 9,
}

impl Card {
    fn value(&self) -> u32 {
        use crate::Card::*;
        match *self {
            A | K | Q | J | T => unsafe { *(self as *const Self as *const u32) },
            Num(num) => {
                if num >= 2 && num <= 9 {
                    num
                } else {
                    panic!()
                }
            }
        }
    }
}

// TODO(ooooooooooooo.......)
// impl PartialEq for Card {
//     fn eq(&self, other: &Self) -> bool {
//         let a = self.value();
//         let b = other.value();
//         a == b
//     }
// }

// TODO(ooooooooooooo.......)
// impl Hash for Card {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let value = self.value();
//         value.hash(state);
//     }
// }

#[repr(u8)]
#[derive(Copy, Clone)]
enum Type {
    FiveKind(u32) = 6,
    FourKind(u32) = 5,
    FullHouse(u32, u32) = 4,
    ThreeKind(u32) = 3,
    TwoPair(u32, u32) = 2,
    Pair(u32) = 1,
    HighCard(u32) = 0,
}

impl Type {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    cards_map: HashMap<Card, u32>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(
            self.hand_type()
                .discriminant()
                .cmp(&other.hand_type().discriminant()),
        )
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        let map = cards.iter().fold(HashMap::new(), |mut acc, card| {
            acc.entry(*card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        });

        Hand {
            cards,
            cards_map: map,
        }
    }

    fn from_str(cards: &str) -> Hand {
        Hand::new(
            cards
                .chars()
                .map(|character| match character {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    num => Card::Num(num.to_digit(10).unwrap()),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn hand_type(&self) -> Type {
        let amount_of_unique_values = self.cards_map.len();
        let cards_clone = self.cards.clone();
        let highest_value_card = cards_clone.iter().max().unwrap();
        let second_highest_value_card = cards_clone
            .iter()
            .filter(|card| *card != highest_value_card)
            .max()
            .unwrap_or(&Card::Num(2));

        let highest_value = highest_value_card.value();

        let mut best_type = Type::HighCard(highest_value);
        let mut pair_amount = 0;

        for (card, count) in self.cards_map.iter() {
            let value = card.value();

            match count {
                5 => best_type = Type::FiveKind(value),
                4 => best_type = Type::FourKind(value),
                3 => {
                    if self.cards_map.len() == 2 {
                        best_type = Type::FullHouse(
                            value,
                            self.cards_map
                                .keys()
                                .find(|other_card| (*(*other_card)).value() != value)
                                .unwrap()
                                .value(),
                        );
                    } else {
                        best_type = Type::ThreeKind(value);
                    }
                }
                2 => {
                    if pair_amount == 0 {
                        pair_amount += 1;
                    } else {
                        best_type = Type::TwoPair(
                            value,
                            self.cards_map
                                .iter()
                                .find(|other_entry| {
                                    (*other_entry.0).value() != value && *other_entry.1 == 2
                                })
                                .unwrap()
                                .0
                                .value(),
                        );
                    }
                }
                _ => (),
            }
        }

        best_type
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Game {
    hand: Hand,
    bet: u32,
}

fn part_one_games_from_input(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand, bet) = line.split_at(line.find(" ").unwrap());
            Game {
                hand: Hand::from_str(hand),
                bet: u32::from_str_radix(bet, 10).unwrap(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut games = part_one_games_from_input(input);

    games.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for game in games {
        println!("{:?}", game.hand);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
