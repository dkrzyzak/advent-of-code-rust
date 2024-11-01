use lazy_static::lazy_static;
use regex::Regex;

use crate::parse_captures;

pub type Move = (u8, u8, u8);

lazy_static! {
    static ref CRATE_REG: Regex = Regex::new(r"").unwrap();
    static ref MOVE_REG: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

pub fn extract_input(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut lines_iter = lines.iter();

    // STACKS PROCESSING
    let mut stack_rows = Vec::new();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let row = process_row(line);
        stack_rows.push(row);
    }

    // remove the line with indexes
    stack_rows.pop();

    // change structure from rows to columns
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_rows[0].len()];

    for row in stack_rows.iter().rev() {
        for (stack_index, option) in row.iter().enumerate() {
            if let Some(letter) = option {
                stacks[stack_index].push(*letter);
            }
        }
    }

    // MOVES PROCESSING
    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = lines_iter.next() {
        let (amount, from, to) = parse_captures!(MOVE_REG, line, 1 u8, 2 u8, 3 u8);
        // subtraction, because we index stacks from 0
        moves.push((amount, from - 1, to - 1));
    }

    (stacks, moves)
}

pub fn process_row(line: &String) -> Vec<Option<char>> {
    let mut chars = line.chars();
    let mut row: Vec<Option<char>> = Vec::new();

    loop {
        let chunk = chars.by_ref().take(4).collect::<String>();
        if chunk.is_empty() {
            break;
        }

        let trimmed = chunk.trim();
        let trimmed_len = trimmed.len();

        let parsed = match trimmed_len {
            0 | 1 => None,
            3 => trimmed.chars().nth(1),
            _ => unreachable!("Unexpected length of crate"),
        };

        row.push(parsed);
    }

    row
}
