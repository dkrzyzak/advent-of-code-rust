use std::time::Instant;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let packages = lines
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .rev()
        .collect::<Vec<_>>();
    let total_size = packages.iter().sum::<u64>();
    let group_size = total_size / 4;

    let mut all_first_groups: Vec<Vec<u64>> = Vec::new();
    let now = Instant::now();

    generate_first_groups(
        group_size as i16,
        Vec::new(),
        &packages,
        0,
        &mut all_first_groups,
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    let group_sizes = all_first_groups
        .iter()
        .map(|group| group.len())
        .collect::<Vec<_>>();
    let smallest_group_size = group_sizes.iter().min().unwrap();
    let smallest_groups = all_first_groups
        .iter()
        .filter(|group| group.len() == *smallest_group_size)
        .map(|group| {
            let entanglement: u64 = group.iter().product();

            entanglement
        })
        .collect::<Vec<_>>();

    let smallest_entanglement = smallest_groups.iter().min().unwrap();

    println!("Smallest entanglement: {:?}", smallest_entanglement);
}

pub fn generate_first_groups(
    target: i16,
    current_packages: Vec<u64>,
    all_packages: &Vec<u64>,
    start_index: usize,
    all_first_groups: &mut Vec<Vec<u64>>,
) {
    if target < 0 {
        return;
    }

    if target == 0 {
        all_first_groups.push(current_packages);
        return;
    }

    for i in start_index..all_packages.len() {
        let package = all_packages[i];
        let mut new_current_packages = current_packages.clone();

        new_current_packages.push(package);
        generate_first_groups(
            target - package as i16,
            new_current_packages,
            all_packages,
            i + 1,
            all_first_groups,
        );
    }
}
