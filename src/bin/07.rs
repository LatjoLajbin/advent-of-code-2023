use itertools::Itertools;

advent_of_code::solution!(7);

enum Type {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    Pair,
    HighCard,
}
struct Hand {
    cards: [char; 5],
}

impl Hand {
    fn Type(&self) -> Type {
        for pair in self.cards.into_iter().combinations(2) {}

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
