use std::cmp::{max, min};

use crate::parse_input;

pub fn task1() {
    let lines = parse_input!();

    let formatted: u32 = lines
        .iter()
        .map(|line| {
            let dims: Vec<u32> = line
                .split("x")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect();

            let (l, w, h) = (dims[0], dims[1], dims[2]);

            let a1 = l * w;
            let a2 = l * h;
            let a3 = w * h;
            let smallest_area = min(a1, min(a2, a3));

            let total_area = 2 * a1 + 2 * a2 + 2 * a3 + smallest_area; // task 1

            let volume = l * w * h;
            let smallest_perimeter = 2 * (l + w + h - max(l, max(w, h)));

            let ribbon_length = smallest_perimeter + volume;

            return ribbon_length;
        })
        .sum();

    println!("Sum of all: {formatted}");
}
