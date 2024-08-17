use regex::Regex;
use super::mappings::*;

pub fn extract_input(lines: &mut Vec<String>) -> (Vec<u64>, Vec<Mapping>) {
    let num_reg = Regex::new(r"(\d+)").unwrap();
    let first_line = lines.first().unwrap();
    let mut cap_iter = num_reg.captures_iter(&first_line);
    let mut seeds: Vec<u64> = Vec::new();

    while let Some(value) = cap_iter.next() {
        let seed = value[1].parse::<u64>().unwrap();
        seeds.push(seed);
    }

    lines.remove(0);
    lines.remove(0);

    let mut lines_iter = lines.iter();
    let mapping_reg = Regex::new(r"(\w+)\-to\-(\w+) map:").unwrap();
    let ranges_reg = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let mut mappings: Vec<Mapping> = Vec::new();

    while let Some(line) = lines_iter.next() {
        if let Some(captured) = mapping_reg.captures(line) {
            let source = captured[1].parse::<String>().unwrap();
            let destination = captured[2].parse::<String>().unwrap();
            let mut ranges = Vec::<Range>::new();

            while let Some(next_line) = lines_iter.next() {
                if next_line == "" {
                    break;
                }

                let captured = ranges_reg.captures(&next_line).unwrap();
                let destination_start = captured[1].parse::<u64>().unwrap();
                let source_start = captured[2].parse::<u64>().unwrap();
                let range_len = captured[3].parse::<u64>().unwrap();
                let range = Range { destination_start, source_start, range_len };
                ranges.push(range);
            }


            let mapping = Mapping { source, destination, ranges };
            mappings.push(mapping);
        }
    }

    (seeds, mappings)
}

pub fn find_location_for_seed(seed: u64, mappings: &Vec<Mapping>) -> u64 {
    let mut current_phase = "seed";
    let mut current_value = seed;

    while current_phase != "location" {
        let mapping = mappings.iter().find(|mapping| mapping.source == current_phase).unwrap();
        current_value = mapping.get_destination(current_value);

        // println!("{} -> {}: {}", current_phase, mapping.destination, current_value);
        
        current_phase = &mapping.destination;
    }

    current_value
}