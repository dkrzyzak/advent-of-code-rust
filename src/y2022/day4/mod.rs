use lazy_static::lazy_static;
use regex::Regex;

use crate::{parse_captures, parse_input};

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

pub fn task() {
    let lines = parse_input!();
    let overlaps = lines
        .iter()
        .map(|line| {
            let (sr1, er1, sr2, er2) = parse_captures!(REG, line, 1 u32, 2 u32, 3 u32, 4 u32);
            
            // PART 1
            // let first_contains_second = sr1 <= sr2 && er1 >= er2;
            // let second_contains_first = sr2 <= sr1 && er2 >= er1;

            // return if first_contains_second || second_contains_first {1} else {0};

            // PART 2
            let no_overlap = er1 < sr2 || er2 < sr1;
            return if no_overlap {0} else {1};
        })
        .sum::<u32>();

        println!("Overlaps: {}", overlaps);

}
