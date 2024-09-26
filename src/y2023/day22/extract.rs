use lazy_static::lazy_static;
use regex::Regex;

use crate::parse_captures;

use super::Brick;

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
}

pub fn extract_input(lines: &Vec<String>) -> Vec<Brick> {
    lines
        .iter()
        .map(|line| {
            let (x1, y1, z1, x2, y2, z2) = parse_captures!(REG, line, 1 isize, 2 isize, 3 isize, 4 isize, 5 isize, 6 isize);
            Brick::new(x1, x2, y1, y2, z1, z2)
        })
        .collect::<Vec<_>>()
}
