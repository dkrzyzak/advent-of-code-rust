use std::collections::HashMap;

use crate::parse_input;

mod hash;
use hash::*;

pub fn task() {
    let lines = parse_input!();
    let sequences = lines.get(0).unwrap().split(",").collect::<Vec<_>>();

    // let total = sequences.iter().map(|sequence| calculate_hash(sequence)).sum::<u64>();
    // println!("Total: {total}");

    let operations = sequences
        .iter()
        .map(|seq| parse_operation(seq))
        .collect::<Vec<_>>();

    let mut hash_map: HashMap<u16, Vec<(String, u8)>> = HashMap::new();

    for operation in operations.iter() {
        match operation {
            Operation::Insert(label, focal_length) => {
                let hash = calculate_hash(&label);

                let entry = hash_map.entry(hash).or_insert_with(Vec::new);
                if let Some(index) = entry.iter().position(|el| el.0 == *label) {
                    entry[index].1 = *focal_length;
                } else {
                    entry.push((label.clone(), *focal_length));
                }
            }
            Operation::Remove(label) => {
                let hash = calculate_hash(&label);
                let entry = hash_map.entry(hash).or_insert_with(Vec::new);
                if let Some(index) = entry.iter().position(|el| el.0 == *label) {
                    entry.remove(index);
                }
            }
        }
    }

    let mut total_focusing_power: u64 = 0;
    for (box_index, sequence) in hash_map.iter() {
        let box_value = *box_index as u64 + 1;

        for (slot_index, (_, focal_length)) in sequence.iter().enumerate() {
            let slot_value = slot_index as u64 + 1;
            let focusing_power = box_value * slot_value * *focal_length as u64;
            total_focusing_power += focusing_power;
        }
    }

    println!("Total focusing power: {}", total_focusing_power);
}
