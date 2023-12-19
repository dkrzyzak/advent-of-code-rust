#![allow(dead_code)]

use std::collections::HashMap;
use super::read::read_input_file;

fn get_num_vec(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .collect()
}

pub fn task1() {
    let lines = read_input_file("day4");

    let x: u32 = lines
        .iter()
        // extract numbers vectors from each line
        .map(|line| {
            let (str1, str2) = line.split_once(" | ").unwrap_or_default();
            let (_, str11) = str1.split_once(": ").unwrap_or_default();

            let winning_numbers = get_num_vec(&str11);
            let my_numbers = get_num_vec(&str2);

            (winning_numbers, my_numbers)
        })
        // count how many numbers were hit
        .map(|(winning_nr, my_nr)| {
            my_nr.iter().fold(0u32, |hit_count, current_nr| {
                if winning_nr.contains(current_nr) {
                    hit_count + 1
                } else {
                    hit_count
                }
            })
        })
        // calculate points for each row
        .map(|x| if x > 1 { u32::pow(2, x - 1) } else { x })
        // sum up points for entire input
        .fold(0u32, |sum, curr| sum + curr);

    dbg!(x);
}

pub fn task2() {
    let lines = read_input_file("day4");

    let wins_vector: Vec<u32> = lines
        .iter()
        // extract numbers vectors from each line
        .map(|line| {
            let (str1, str2) = line.split_once(" | ").unwrap_or_default();
            let (_, str11) = str1.split_once(": ").unwrap_or_default();

            let winning_numbers = get_num_vec(&str11);
            let my_numbers = get_num_vec(&str2);

            (winning_numbers, my_numbers)
        })
        // count how many numbers were hit
        .map(|(winning_nr, my_nr)| {
            my_nr.iter().fold(0u32, |hit_count, current_nr| {
                if winning_nr.contains(current_nr) {
                    hit_count + 1
                } else {
                    hit_count
                }
            })
        })
        .collect();


    let mut cards_won: HashMap<usize, u32> = HashMap::new();

   // populate map with one initial card for each index
    for i in 0..wins_vector.len() {
      cards_won.insert(i, 1);
    }

    for (index, &wins) in wins_vector.iter().enumerate() {
        for offset in 0..(wins) as usize {
            let next_card_index = index + offset + 1;
            let won_count = cards_won.get(&index).cloned().unwrap_or(0);

            cards_won
                .entry(next_card_index)
                .and_modify(|existing_cards| *existing_cards += won_count)
                .or_insert(won_count);
        }
    }

    let total = cards_won.iter().fold(0, |sum, curr| sum + (curr).1);
    dbg!(total);
}
