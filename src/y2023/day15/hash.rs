use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REMOVE_REG: Regex = Regex::new(r"(\w+)\-").unwrap();
    static ref INSERT_REG: Regex = Regex::new(r"(\w+)=(\d)").unwrap();
}


pub fn calculate_hash(sequence: &str) -> u16 {
    let mut current_value: u16 = 0;

    for ch in sequence.chars() {
        let ascii_value = ch as u8;
        current_value += ascii_value as u16;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
#[derive(Debug)]
pub enum Operation {
    Remove(String),
    Insert(String, u8),
}

pub fn parse_operation(sequence: &str) -> Operation {
    if let Some(captured) = REMOVE_REG.captures(sequence) {
        let label = captured[1].parse::<String>().unwrap();
        return Operation::Remove(label);
    }

    let captured = INSERT_REG.captures(sequence).unwrap();
    let label = captured[1].parse::<String>().unwrap();
    let fl = captured[2].parse::<u8>().unwrap();
    return Operation::Insert(label, fl);
}
