use std::collections::HashMap;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();

    let (mut first_list, mut second_list): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|line| {
            let mut splitted = line.split_whitespace();
            let first = splitted.next().unwrap();
            let second = splitted.next().unwrap();

            let first_num = first.parse::<u32>().unwrap();
            let second_num = second.parse::<u32>().unwrap();

            (first_num, second_num)
        })
        .unzip();

    // PART 1
    first_list.sort();
    second_list.sort();

    let len = first_list.len();
    let mut total_diff = 0u32;
    for index in 0..len {
        total_diff += first_list[index].abs_diff(second_list[index]);
    }

    println!("Total diff: {}", total_diff);

    // PART 2
    let mut frequency_map = HashMap::<u32, u32>::new();
    for item in second_list.iter() {
        *frequency_map.entry(*item).or_insert(0) += 1;
    }

    let similarity_score = first_list
        .iter()
        .map(|&item| item * frequency_map.get(&item).unwrap_or(&0))
        .sum::<u32>();

    println!("Similarity score: {}", similarity_score);
}
