use regex::Regex;

use crate::{parse_captures, parse_input};

#[derive(Debug)]
pub struct Input(char, u8, String);

pub fn task() {
    let lines = parse_input!();
    let instr = extract_instructions(&lines);

    println!("{:?}", instr);
}

pub fn extract_instructions(lines: &Vec<String>) -> Vec<Input> {
    let reg = Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap();

    lines.iter().map(|line| {
        let (dir, steps, hex): (char, u8, String) = parse_captures!(&reg, line, 1 char, 2 u8, 3 String);

        Input(dir, steps, hex)
    }).collect::<Vec<_>>()

}