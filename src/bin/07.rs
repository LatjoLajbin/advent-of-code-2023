use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

advent_of_code::solution!(7);

trait Card {
    fn value(&self) -> u32;
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
enum JackCard {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Num(u32) = 9,
    None = 0,
}

impl Card for JackCard {
    fn value(&self) -> u32 {
        use crate::JackCard::*;
        match *self {
            A | K | Q | J | T => unsafe { *(self as *const Self as *const u32) },
            Num(num) => {
                if num >= 2 && num <= 9 {
                    num
                } else {
                    panic!()
                }
            }
            None => panic!(),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy, PartialOrd)]
enum JokerCard {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Num(u32) = 9,
    J = 1,
    None = 0,
}

impl Card for JokerCard {
    fn value(&self) -> u32 {
        use crate::JokerCard::*;
        match *self {
            A | K | Q | T => unsafe { *(self as *const Self as *const u32) },
            Num(num) => {
                if num >= 2 && num <= 9 {
                    num
                } else {
                    panic!()
                }
            }
            J => 1,
            None => panic!(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum HandType {
    FiveKind(JackCard) = 6,
    FourKind(JackCard) = 5,
    FullHouse(JackCard, JackCard) = 4,
    ThreeKind(JackCard) = 3,
    TwoPair(JackCard, JackCard) = 2,
    Pair(JackCard) = 1,
    HighCard(JackCard) = 0,
}

impl HandType {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn value(&self) -> (JackCard, JackCard) {
        use crate::HandType::*;
        match self {
            FiveKind(val) | FourKind(val) | ThreeKind(val) | Pair(val) | HighCard(val) => {
                (*val, JackCard::None)
            }
            FullHouse(val1, val2) | TwoPair(val1, val2) => (*val1, *val2),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand<T>
where
    T: Card + PartialEq,
{
    cards: [T; 5],
    hand_type: HandType,
    card_occurences: HashMap<T, u32>,
    cards_not_in_type: HashSet<T>,
    cards_string: String,
}

impl PartialOrd for Hand<JokerCard> {
    fn partial_cmp(&self, other: &Hand<JokerCard>) -> Option<Ordering> {
        None
    }
}

impl PartialOrd for Hand<JackCard> {
    fn partial_cmp(&self, other: &Hand<JackCard>) -> Option<Ordering> {
        let self_type = self.hand_type;
        let self_value = self_type.value();
        let self_discriminant = self_type.discriminant();

        let other_type = other.hand_type;
        let other_value = other_type.value();
        let other_discriminant = other_type.discriminant();

        let card_by_card = true;

        if self_discriminant == other_discriminant {
            if card_by_card {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(a, b)| (a.value(), b.value()))
                    .filter(|(a, b)| a != b)
                    .find(|(_, _)| true)
                    .map(|(a, b)| a.cmp(&b))
            } else if self_value == other_value {
                let self_cards_not_in_type = self.cards.clone();
                let other_cards_not_in_type = other.cards.clone();

                let mut self_cards_not_in_type: Vec<u32> = self_cards_not_in_type
                    .iter()
                    .filter(|card| **card != self_value.0 && **card != self_value.1)
                    .map(|card| card.value())
                    .collect();
                let mut other_cards_not_in_type: Vec<_> = other_cards_not_in_type
                    .iter()
                    .filter(|card| **card != other_value.0 && **card != other_value.1)
                    .map(|card| card.value())
                    .collect();

                self_cards_not_in_type.sort();
                other_cards_not_in_type.sort();

                let mut ordering: Option<Ordering> = Some(Ordering::Equal);

                let _ordering = self_cards_not_in_type
                    .iter()
                    .rev()
                    .zip(other_cards_not_in_type.iter().rev())
                    .for_each(|(self_value, other_value)| {
                        if ordering == Some(Ordering::Equal) {
                            let comparison = self_value.cmp(&other_value);

                            if comparison != Ordering::Equal {
                                ordering = Some(comparison);
                            }
                        }
                    });

                ordering
            } else {
                Some(self_value.cmp(&other_value))
            }
        } else {
            Some(self_discriminant.cmp(&other_discriminant))
        }
    }
}

impl Hand<JackCard> {
    fn new(cards: [JackCard; 5], as_str: &str) -> Hand<JackCard> {
        let map = cards.iter().fold(HashMap::new(), |mut acc, card| {
            acc.entry(*card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        });

        let hand_type = Hand::get_hand_type(cards, &map);

        Hand {
            cards,
            hand_type,
            card_occurences: map,
            cards_not_in_type: HashSet::new(),
            cards_string: String::from(as_str),
        }
    }

    fn from_str_jacks(cards: &str) -> Hand<JackCard> {
        Hand::new(
            cards
                .chars()
                .map(|character| match character {
                    'A' => JackCard::A,
                    'K' => JackCard::K,
                    'Q' => JackCard::Q,
                    'J' => JackCard::J,
                    'T' => JackCard::T,
                    num => JackCard::Num(num.to_digit(10).unwrap()),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            cards,
        )
    }

    fn get_hand_type(cards: [JackCard; 5], card_occurences: &HashMap<JackCard, u32>) -> HandType {
        let highest_value_card = cards.iter().max().unwrap();

        let mut best_type = HandType::HighCard(*highest_value_card);
        let mut pair_amount = 0;

        for (card, count) in card_occurences.iter() {
            let value = card.value();
            let discriminant = best_type.discriminant();

            match count {
                5 => best_type = HandType::FiveKind(*card),
                4 => best_type = HandType::FourKind(*card),
                3 if discriminant <= 3 => {
                    if card_occurences.len() == 2 {
                        let other_card = *card_occurences
                            .keys()
                            .find(|other_card| *other_card != card)
                            .unwrap();

                        if value > other_card.value() {
                            best_type = HandType::FullHouse(*card, other_card);
                        } else {
                            best_type = HandType::FullHouse(other_card, *card);
                        }
                    } else if discriminant < 4 {
                        best_type = HandType::ThreeKind(*card);
                    }
                }
                2 if discriminant <= 2 => {
                    if pair_amount == 0 {
                        pair_amount += 1;
                        best_type = HandType::Pair(*card);
                    } else {
                        let other_card = *card_occurences
                            .iter()
                            .find(|other_entry| (*other_entry.0) != *card && *other_entry.1 == 2)
                            .unwrap()
                            .0;

                        if value > other_card.value() {
                            best_type = HandType::TwoPair(*card, other_card);
                        } else {
                            best_type = HandType::TwoPair(other_card, *card);
                        }
                    }
                }
                _ => (),
            }
        }

        best_type
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Game<T>
where
    T: Card + PartialEq + PartialOrd,
{
    hand: Hand<T>,
    bet: u32,
}

fn part_one_games_from_input(input: &str) -> Vec<Game<JackCard>> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand, bet) = line.split_at(line.find(" ").unwrap());
            let bet = bet.trim();
            Game {
                hand: Hand::from_str(hand.trim()),
                bet: u32::from_str_radix(bet, 10).unwrap(),
            }
        })
        .collect()
}

fn part_two_games_from_input(input: &str) -> Vec<Game<JokerCard>> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand, bet) = line.split_at(line.find(" ").unwrap());
            let bet = bet.trim();
            Game {
                hand: Hand::from_str(hand.trim()),
                bet: u32::from_str_radix(bet, 10).unwrap(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut games = part_one_games_from_input(input);

    games.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // for (index, game) in games[20..100].iter().enumerate() {
    // for (index, game) in games.iter().enumerate() {
    //     let mut sorted_cards = game.hand.cards.clone();
    // sorted_cards.sort_by(|a, b| b.cmp(&a));

    // let rank: u32 = (index as u32) + 1;
    // let hand_type = game.hand.hand_type;
    // let hand_discriminant = hand_type.discriminant();
    // let hand_value = hand_type.value();
    // let adding = game.bet * rank;
    // println!("TYPE: {:?} === STRING: {:?}", hand_type, sorted_cards);
    // println!("hand as string: {:?}", game.hand.cards_string);
    // println!("hand: {:?}", game.hand);
    // println!("hand type: {:?}", hand_type);
    // println!("hand type discriminant: {:?}", hand_discriminant);
    // println!("hand type value: {:?}", hand_value);
    // println!("adding: {} * {} = {}", game.bet, rank, adding);
    // println!("");
    // }
    // let total_winnings = games.iter().enumerate().fold(0, |mut acc, (index, game)| {
    //     let rank: u32 = (index as u32) + 1;
    //     let adding = game.bet * rank;
    //     println!("rank: {}", rank);
    //     println!("bet: {}", game.bet);
    //     println!("adding: {} * {} = {}", game.bet, rank, adding);
    //     println!("current winnings: {}", acc);
    //     println!("");
    //     acc += game.bet * rank;
    //     acc
    // });

    Some(
        games
            .iter()
            .enumerate()
            .map(|(index, game)| game.bet * ((index as u32) + 1))
            .sum(),
    )
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games = part_two_games_from_input(input);

    games.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // for (index, game) in games[20..100].iter().enumerate() {
    // for (index, game) in games.iter().enumerate() {
    //     let mut sorted_cards = game.hand.cards.clone();
    // sorted_cards.sort_by(|a, b| b.cmp(&a));

    // let rank: u32 = (index as u32) + 1;
    // let hand_type = game.hand.hand_type;
    // let hand_discriminant = hand_type.discriminant();
    // let hand_value = hand_type.value();
    // let adding = game.bet * rank;
    // println!("TYPE: {:?} === STRING: {:?}", hand_type, sorted_cards);
    // println!("hand as string: {:?}", game.hand.cards_string);
    // println!("hand: {:?}", game.hand);
    // println!("hand type: {:?}", hand_type);
    // println!("hand type discriminant: {:?}", hand_discriminant);
    // println!("hand type value: {:?}", hand_value);
    // println!("adding: {} * {} = {}", game.bet, rank, adding);
    // println!("");
    // }
    // let total_winnings = games.iter().enumerate().fold(0, |mut acc, (index, game)| {
    //     let rank: u32 = (index as u32) + 1;
    //     let adding = game.bet * rank;
    //     println!("rank: {}", rank);
    //     println!("bet: {}", game.bet);
    //     println!("adding: {} * {} = {}", game.bet, rank, adding);
    //     println!("current winnings: {}", acc);
    //     println!("");
    //     acc += game.bet * rank;
    //     acc
    // });

    Some(
        games
            .iter()
            .enumerate()
            .map(|(index, game)| game.bet * ((index as u32) + 1))
            .sum(),
    )
    // None
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
