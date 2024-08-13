#![allow(dead_code)]

use crate::parse_input;

fn extract_numbers(input: &str) -> Vec<u32> {
    let digit_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            result.push(digit);
            current_number.clear();
        } else {
            current_number.push(c);

            if let Some(index) = digit_words.iter().position(|&word| current_number.contains(word)) {
               result.push(index as u32 + 1);
               current_number.clear();

                // to handle threeight, eightree, sevenine, twone and similar cases
               let last_letter = match index {
                    0 => Some('e'), // onE
                    1 => Some('o'), // twO
                    2 => Some('e'), // threE
                    3 => Some('r'), // ...and so on
                    4 => Some('e'),
                    5 => Some('x'),
                    6 => Some('n'),
                    7 => Some('t'),
                    8 => Some('e'),
                    _ => Some('\0'),
               };

               current_number.push(last_letter.unwrap());
            }
        }
    }

    if let Some(index) = digit_words.iter().position(|&word| current_number.contains(word)) {
        result.push(index as u32 + 1);
    }

    result
}

pub fn task1() {
    let lines = parse_input!();

    let calibration_values: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| extract_numbers(&line) )
        .collect();

    let total = calibration_values.iter().fold(0, |sum, current| {
        let first = current.first().cloned().unwrap_or_default();
        let last = current.last().cloned().unwrap_or_default();

        let num_u32: u32 = format!("{first}{last}").parse().unwrap();

        sum + num_u32
    });

    println!("SUM: {total}");
}
