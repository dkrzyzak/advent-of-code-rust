use std::{collections::HashMap, time::Instant};

use crate::{algos::Between, parse_input};

#[derive(Debug)]
struct Pair {
    a: u32,
    b: u32,
    distance: u128,
}

impl Pair {
    pub fn new(a: u32, b: u32, distance: u128) -> Self {
        Pair { a, b, distance }
    }

    pub fn contains(&self, a: u32, b: u32) -> bool {
        (self.a == a && self.b == b) || (self.a == b && self.b == a)
    }
}

pub fn task() {
    let mut lines: Vec<Vec<char>> = parse_input!()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let (empty_rows, empty_cols) = locate_empty_spaces(&mut lines);

    // gather galaxies into a map of u32 (index) -> (row, col)
    let mut galaxies: HashMap<u32, (usize, usize)> = HashMap::new();

    for (row_index, row) in lines.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.insert(galaxies.len() as u32, (row_index, col_index));
            }
        }
    }

    let mut pairs: Vec<Pair> = Vec::new();
    let growth_factor = 1_000_000 - 1; // 1 is included in the calculations

    let now = Instant::now();

    for (a, a_coords) in galaxies.iter() {
        for (b, b_coords) in galaxies.iter() {
            if a != b && !pairs.iter().any(|pair| pair.contains(*a, *b)) {
                let smaller_row = a_coords.0.min(b_coords.0);
                let bigger_row = a_coords.0.max(b_coords.0);
                let smaller_col = a_coords.1.min(b_coords.1);
                let bigger_col = a_coords.1.max(b_coords.1);

                let distance = (bigger_row - smaller_row) + (bigger_col - smaller_col);
                let mut expanded_distance: u128 = 0;

                for empty_row in empty_rows.iter() {
                    if empty_row.between(smaller_row, bigger_row) {
                        expanded_distance += growth_factor;
                    }
                }

                for empty_col in empty_cols.iter() {
                    if empty_col.between(smaller_col, bigger_col) {
                        // println!("{} is between {} {} in pairs {} and {}", empty_col, smaller_row, bigger_row, a, b);
                        expanded_distance += growth_factor;
                    }
                }

                let total_distance = distance as u128 + expanded_distance;
                // println!("Distance between {:?} and {:?} is {:?}", a, b, total_distance);
                pairs.push(Pair::new(*a, *b, total_distance));
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Counted all the distances in {:?}", elapsed);

    // println!("Pairs len: {:?}", pairs.len());

    let total_distance: u128 = pairs
        .iter()
        .fold(0, |total, current_pair| total + current_pair.distance);
    println!("Total distance: {:?}", total_distance);
}

fn locate_empty_spaces(universe: &mut Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for row in 0..universe.len() {
        if universe[row].iter().all(|&c| c == '.') {
            empty_rows.push(row);
        }
    }

    for col in 0..universe[0].len() {
        if universe.iter().all(|row| row[col] == '.') {
            empty_cols.push(col);
        }
    }

    println!("Empty rows: {:?}", empty_rows);
    println!("Empty cols: {:?}", empty_cols);

    (empty_rows, empty_cols)
}
