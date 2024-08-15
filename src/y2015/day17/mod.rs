use itertools::Itertools;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let containers = lines
        .iter()
        .map(|line| line.parse::<u16>().unwrap())
        .sorted_by(|a, b| b.cmp(a)) // must be sorted in a descending order
        .collect::<Vec<u16>>();

    let target = 150;
    let mut all_combinations = Vec::new();

    find_combinations(target, &containers, Vec::new(), 0, &mut all_combinations);

    // for combination in &all_combinations {
    //     println!("{:?}", combination);
    // }

    println!("All combinations: {}", all_combinations.len());

    let smallest_combination_amount = all_combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();

    println!("Smallest combination: {}", smallest_combination_amount);

    let smallest_combinations = all_combinations
        .iter()
        .filter(|combination| combination.len() == smallest_combination_amount)
        .collect::<Vec<_>>();

    println!(
        "Smallest combinations amount: {}",
        smallest_combinations.len()
    );
}

fn find_combinations(
    target: i16,
    containers: &Vec<u16>,
    current_combination: Vec<u16>,
    start_index: usize,
    all_combinations: &mut Vec<Vec<u16>>,
) {
    if target == 0 {
        // proper combination
        all_combinations.push(current_combination);
        return;
    }

    if target < 0 {
        // wrong combination
        return;
    }

    for i in start_index..containers.len() {
        let container = *(&containers[i]);
        let mut new_combination = current_combination.clone();
        new_combination.push(container);

        find_combinations(
            target - container as i16,
            containers,
            new_combination,
            i + 1,
            all_combinations,
        );
    }
}
