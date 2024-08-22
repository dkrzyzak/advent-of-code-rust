use std::time::Instant;

use crate::parse_input;

mod tilt;
mod cycle;

use tilt::*;
use cycle::*;

pub fn task() {
    let lines = parse_input!();

    let mut mirror = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let now = Instant::now();
    let iterations_count = 1000000000;

    // it will eventually loop, so we only need to figure out when
    let mut loads: Vec<usize> = Vec::new();

    for _ in 0..1000 {
        tilt_north(&mut mirror);
        tilt_west(&mut mirror);
        tilt_south(&mut mirror);
        tilt_east(&mut mirror);

        let total_load = calculate_total_load(&mirror);
        loads.push(total_load);
    }

    let (cycle, cycle_offset) = find_cycle(&loads).expect("Didn't find cycle...");
    let total_load = get_load_on_iteration(iterations_count, cycle, cycle_offset);

    println!("Total load: {}", total_load);

    // print_mirror(&mirror);

    println!("Done in {:?}", now.elapsed());
}

pub fn print_mirror(mirror: &Vec<Vec<char>>) {
    for row in mirror.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}

pub fn calculate_total_load(mirror: &Vec<Vec<char>>) -> usize {
    let rows = mirror.len();

    mirror
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let weight = rows - index;
            let total_rocks = line.iter().filter(|ch| **ch == 'O').count();

            total_rocks * weight
        })
        .sum()
}

