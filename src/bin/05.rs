#![feature(iter_map_windows)]
use indexmap::IndexMap;

advent_of_code::solution!(5);

struct Mapping {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

fn get_seeds(rows: &Vec<String>) -> Vec<usize> {
    rows.first()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter_map(|seed| seed.parse::<usize>().ok())
        .collect()
}

fn get_seeds_as_range(rows: &Vec<String>) -> Vec<Vec<usize>> {
    let seeds = get_seeds(rows);
    println!("Preparing to get seeds.");

    seeds
        .chunks(2)
        .map(|current_chunk| {
            let seed_number = current_chunk[0];
            let range_length = current_chunk[1];
            Vec::from_iter(seed_number..seed_number + range_length)
        })
        .collect()
}

fn get_mappings<'a>(
    rows: &Vec<String>,
    mapping_headers: Vec<&'a str>,
) -> IndexMap<&'a str, Vec<Mapping>> {
    // Ignore first line, which contains seeds.
    // Then, ignore one more since every header is followed by a newline
    let mut last_processed_row_index: usize = 2;
    mapping_headers
        .into_iter()
        .fold(IndexMap::new(), |mut accumulator, current_header| {
            accumulator.insert(
                current_header,
                rows.iter()
                    .enumerate()
                    .skip(last_processed_row_index)
                    .take_while(|(row_index, row)| match row.is_empty() {
                        true => {
                            last_processed_row_index = *row_index + 1;
                            false
                        }
                        _ => true,
                    })
                    .map(|(_row_index, row)| {
                        row.split(" ")
                            .filter_map(|number| number.parse::<usize>().ok())
                            .collect()
                    })
                    .filter(|numbers_in_row: &Vec<usize>| !numbers_in_row.is_empty())
                    .map(|numbers_in_row| Mapping {
                        destination_range_start: numbers_in_row[0],
                        source_range_start: numbers_in_row[1],
                        range_length: numbers_in_row[2],
                    })
                    .collect(),
            );
            accumulator
        })
}

fn remap_value(source: usize, map: &Vec<Mapping>) -> usize {
    for m in map {
        let above = source > m.source_range_start;
        let below = source < m.source_range_start + m.range_length;

        if above && below {
            return m.destination_range_start + (source - m.source_range_start);
        }
    }

    source
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
    let seeds = get_seeds(&rows);

    let mappings = get_mappings(
        &rows,
        vec![
            "seed_to_soil_map:",
            "soil-to-fertilizer map:",
            "fertilizer-to-water map:",
            "water-to-light map:",
            "light-to-temperature map:",
            "temperature-to-humidity map:",
            "humidity-to-location map:",
        ],
    );

    let mut locations = seeds.clone();

    for (index, _seed) in seeds.iter().enumerate() {
        for map in mappings.values() {
            locations[index] = remap_value(locations[index], map);
        }
    }

    let location_with_lowest_value = *locations.iter().min().unwrap();
    Some(location_with_lowest_value as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
    let seeds = get_seeds_as_range(&rows);
    println!("Got seeds.");

    let mappings = get_mappings(
        &rows,
        vec![
            "seed_to_soil_map:",
            "soil-to-fertilizer map:",
            "fertilizer-to-water map:",
            "water-to-light map:",
            "light-to-temperature map:",
            "temperature-to-humidity map:",
            "humidity-to-location map:",
        ],
    );
    println!("Got mappings.");

    let mut locations = seeds.clone();

    println!("Preparing to go through {} seed ranges.", seeds.len());
    for (index, seed_range) in seeds.iter().enumerate() {
        println!("Preparing to go through {} seeds.", seed_range.len());
        for (inner_index, _seed) in seed_range.iter().enumerate() {
            for map in mappings.values() {
                locations[index][inner_index] = remap_value(locations[index][inner_index], map);
            }
        }
    }

    println!("Mapped seeds.");

    let location_with_lowest_value = *locations
        .iter()
        .flat_map(|outer| outer.iter())
        .min()
        .unwrap();
    Some(location_with_lowest_value as u32)
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
