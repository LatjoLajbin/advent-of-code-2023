use core::hash::Hash;
use std::collections::HashSet;

advent_of_code::solution!(7);

#[repr(u32)]
#[derive(PartialEq, Eq)]
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
impl PartialEq for MyEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MyEnum::Var3(a, _), MyEnum::Var3(b, _)) => a == b,
            _ => format!("{:?}", self) == format!("{:?}", other),
        }
    }
}

// TODO(ooooooooooooo.......)
impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MyEnum::Var3(a, _) => a.hash(state),
            _ => format!("{:?}", self).hash(state),
        }
    }
}

enum Type {
    FiveKind(u32),
    FourKind(u32),
    FullHouse(u32, u32),
    ThreeKind(u32),
    TwoPair(u32, u32),
    Pair(u32),
    HighCard(u32),
}
struct Hand {
    cards: [Card; 5],
    cards_set: HashSet<Card>,
}

impl Hand {
    fn build(cards: [Card; 5]) -> Hand {
        let set = cards.iter().fold(HashSet::new(), |mut acc, card| {
            acc.insert(*card);
            acc
        });

        Hand {
            cards,
            cards_set: set,
        }
    }

    fn hand_type(&self) -> Type {
        let amount_of_unique_values = self.cards_set.len();
        let cards_clone = self.cards.clone();
        let highest_value = cards_clone.iter().max().unwrap();
        let second_highest_value = cards_clone
            .iter()
            .filter(|card| *card != highest_value)
            .max()
            .unwrap_or(&'0');

        if amount_of_unique_values == 1 {
            Type::HighCard
        } else if amount_of_unique_values == 2 {
            Type::FourKind
        }

        Type::HighCard
    }

    fn five_kind(&self) -> bool {
        match self.cards {
            [head, tail @ ..] => tail.iter().all(|x| *x == head),
            _ => false,
        }
    }

    fn four_kind(&self) -> bool {
        self.amount_of_matching_cards() == 4
    }

    fn house(&self) -> bool {
        false
    }
    fn three_kind(&self) -> bool {
        self.amount_of_matching_cards() == 3
    }
    fn two_pair(&self) -> bool {
        false
    }
    fn pair(&self) -> bool {
        false
    }
    fn high_card(&self) -> bool {
        false
    }

    fn amount_of_matching_cards(&self) -> u32 {
        let mut matches = 0;
        for i in 0..self.cards.len() {
            let mut current_matches = 0;
            for j in 0..self.cards.len() {
                if i == j {
                    continue;
                }
                if self.cards[i] == self.cards[j] {
                    current_matches = current_matches + 1;
                }
            }

            if current_matches > matches {
                matches = current_matches;
            }
        }

        matches
        // match self.cards {
        //     [head, tail @ ..] => tail.iter().enumerate().ma(|x| *x == head),
        //     _ => false,
        // }
        // match self.cards {
        //     [all @ ..] => match all {
        //         [head, tail @ ..] => tail.iter().fold(0, |mut matches, current| {
        //         if current == head {
        //             matches = matches + 1;
        //         }
        //     }),
        //     _ => 0,
        // }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
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
