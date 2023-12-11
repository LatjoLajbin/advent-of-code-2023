use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug)]
struct BoatRace {
    time: u64,
    record: u64,
}

fn part_one_numbers_from_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .map(|line| {
            line.split(" ")
                .filter_map(|word| word.parse::<u64>().ok())
                .collect()
        })
        .filter(|line: &Vec<u64>| !line.is_empty())
        .collect()
}

fn part_two_numbers_from_input(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .map(|line| {
            line.trim()
                .split(":")
                .skip(1)
                .filter_map(|word| {
                    word.split_whitespace()
                        .into_iter()
                        .join("")
                        .trim()
                        .parse::<u64>()
                        .ok()
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn races_from_numbers(numbers: &Vec<Vec<u64>>) -> Vec<BoatRace> {
    // println!("{:#?}", numbers);
    (*numbers)
        .chunks(2)
        .map(|chunk| {
            chunk[0]
                .iter()
                .zip(chunk[1].iter())
                .map(|(time, record)| BoatRace {
                    time: *time,
                    record: *record,
                })
        })
        .flatten()
        .collect()
}

fn get_distance(time: u64, charge_time: u64) -> u64 {
    (time - charge_time) * charge_time
}

fn get_charge_times_faster_than_record(race: &BoatRace) -> u64 {
    (0..race.time)
        .map(|charge_time| get_distance(race.time, charge_time))
        .filter(|distance| *distance > race.record)
        // .inspect(|num| println!("This number should be higher than 233: {}", num))
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = part_one_numbers_from_input(input);
    let races = races_from_numbers(&numbers);

    Some(
        races
            .iter()
            .map(|race| get_charge_times_faster_than_record(race))
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = part_two_numbers_from_input(input);

    let (time, record) = numbers.into_iter().tuples().next().unwrap();
    let race = BoatRace { time, record };

    Some(get_charge_times_faster_than_record(&race))
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
