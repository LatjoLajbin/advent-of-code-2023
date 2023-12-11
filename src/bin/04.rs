advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    index: usize,
    winning_numbers: Vec<usize>,
    numbers_on_card: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = read_cards(input);

    let mut total_points = 0;

    for card in cards {
        total_points += get_points_from_card(&card);
    }

    Some(total_points as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_points_from_card(card: &Card) -> usize {
    let mut winning_numbers_on_card = 0;

    for card_number in card.numbers_on_card.iter() {
        if card.winning_numbers.contains(card_number) {
            winning_numbers_on_card += 1;
        }
    }

    // println!("Card {}: {} cards", (*card).index, winning_numbers_on_card);

    let base: usize = 2;
    let points = if winning_numbers_on_card > 0 {
        base.pow(winning_numbers_on_card - 1)
    } else {
        0
    };
    // println!("Card {}: {} points", (*card).index, points);
    points
}

fn read_cards(input: &str) -> Vec<Card> {
    let mut v: Vec<Card> = vec![];

    let rows: Vec<String> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    for (row_index, row) in rows.iter().enumerate() {
        let winning_numbers_start_index = row.chars().position(|c| c == ':').unwrap() + 2;
        let winning_numbers_end_index = row.chars().position(|c| c == '|').unwrap();
        let winning_numbers_slice =
            String::from(&row[winning_numbers_start_index..winning_numbers_end_index]);
        let winning_numbers_parsed: Vec<usize> = winning_numbers_slice
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let numbers_on_card_start_index = winning_numbers_end_index + 2;
        let numbers_on_card_slice = String::from(&row[numbers_on_card_start_index..]);
        let numbers_on_card_parsed: Vec<usize> = numbers_on_card_slice
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let new_card = Card {
            index: row_index,
            winning_numbers: winning_numbers_parsed,
            numbers_on_card: numbers_on_card_parsed,
        };

        // println!("index: {:?}", new_card.index);
        // println!("winning cards: {:?}", new_card.winning_numbers);
        // println!("numbers on card: {:?}", new_card.numbers_on_card);

        v.push(new_card);
    }

    return v;
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
