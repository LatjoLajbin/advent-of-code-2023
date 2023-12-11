advent_of_code::solution!(6);

struct BoatRace {
    time: u128,
    record: u128,
}

fn numbers_from_input(input: &str) -> Vec<Vec<u128>> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .map(|line| {
            line.split(" ")
                .filter_map(|word| word.parse::<u128>().ok())
                .collect()
        })
        .filter(|line: &Vec<u128>| !line.is_empty())
        .collect()
}

fn races_from_numbers(numbers: &Vec<Vec<u128>>) -> Vec<BoatRace> {
    println!("{:#?}", numbers);
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

fn get_distance(time: u128, charge_time: u128) -> u128 {
    (time - charge_time) * charge_time
}

fn get_charge_times_faster_than_record(race: &BoatRace) -> Vec<u128> {
    //Filter<Map<std::ops::Range<u128>>, _> {
    (0..race.time)
        .map(|charge_time| get_distance(race.time, charge_time))
        .filter(|distance| *distance > race.record)
        .collect()
}

pub fn part_one(input: &str) -> Option<u128> {
    let numbers = numbers_from_input(input);
    let races = races_from_numbers(&numbers);

    Some(
        races
            .iter()
            .map(|race| get_charge_times_faster_than_record(race))
            .flatten()
            .inspect(|num| println!("{}", num))
            .product(),
    )

    // for (index, race) in races.iter().enumerate() {
    //     println!("Getting values for race {}, where the time limit is {} ms and where the record is {} ms.", index, race.time, race.record);
    //     let values = get_charge_times_faster_than_record(race);
    //     println!("Times faster than record: {:#?}", values);
    //     println!("Done.\n");
    // }
    // None
}

pub fn part_two(input: &str) -> Option<u128> {
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
