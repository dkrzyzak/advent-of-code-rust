use helpers::*;
use regex::Regex;
use std::collections::HashMap;

use crate::parse_input;

mod helpers;

pub fn task() {
    let lines = parse_input!();
    let springs_data = extract_input(&lines);
    let mut total_possible_combinations: u128 = 0;
    let mut iteration_count = 0;
    let mut cache: HashMap<(String, Vec<u8>), u128> = HashMap::new();
    let splitter = Regex::new(r"0+").unwrap();

    for springs in springs_data.iter() {
        println!("Item nr {}", iteration_count);
        iteration_count += 1;

        total_possible_combinations +=
            get_correct_combinations(springs.0.clone(), springs.1.clone(), &splitter, &mut cache);
    }

    println!(
        "Total correct combinations: {}",
        total_possible_combinations
    );
}

fn get_correct_combinations(
    springs: String,
    good_springs: Vec<u8>,
    splitter: &Regex,
    cache: &mut HashMap<(String, Vec<u8>), u128>,
) -> u128 {
    // we got to the final version
    if !springs.contains("x") {
        let correct_regex = get_correct_regex(&good_springs);
        if correct_regex.is_match(&springs) {
            return 1;
        }

        return 0;
    }

    // let splitted = splitter
    //     .split(&springs)
    //     .filter(|s| !s.is_empty())
    //     .map(|s| s.len() as u8)
    //     .collect::<Vec<_>>();

    let acceptable_regex = get_acceptable_regex(&good_springs);

    if !acceptable_regex.is_match(&springs) {
        return 0;
    }

    let replaced_with_0 = springs.replacen("x", "0", 1);
    let replaced_with_1 = springs.replacen("x", "1", 1);

    return get_correct_combinations(replaced_with_0, good_springs.clone(), splitter, cache)
        + get_correct_combinations(replaced_with_1, good_springs.clone(), splitter, cache);
}