use itertools::Itertools;

use extract_matrix::extract_matrix_from_input;
use printers::print_happiness_matrix;
use crate::parse_input;

mod extract_matrix;
mod printers;

pub fn task() {
    let lines = parse_input!();
    let people_count = 9;
    let (matrix, names) = extract_matrix_from_input(&lines, people_count);

    print_happiness_matrix(&matrix, &names);

    let permutations: Vec<Vec<&String>> = names.iter().permutations(names.len()).collect();

    let max = permutations.iter().map(|perm| calculate_happiness(&perm, &matrix, &names)).max().unwrap();

    println!("Max hap: {:?}", max);
}

fn calculate_happiness(table: &Vec<&String>, happiness_matrix: &Vec<Vec<i16>>, original_names: &Vec<String>) -> i16 {
    let len = table.len();
    let mut score: i16 = 0;

    for i in 0..len {
        let curr = &table[i];
        let neighbour = &table[(i + 1) % len];

        let curr_index = original_names.iter().position(|name| name == *curr).unwrap();
        let neighbour_index = original_names.iter().position(|name| name == *neighbour).unwrap();

        let hp_1 = happiness_matrix[curr_index][neighbour_index];
        let hp_2 = happiness_matrix[neighbour_index][curr_index];

        score += hp_1;
        score += hp_2;
    }

    score
}