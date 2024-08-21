use crate::parse_input;

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let mirrors = extract_mirrors(&lines);

    let mut result: usize = 0;

    for mirror in mirrors.iter() {
        if let Some(horizontal_index) = horizontal_index(&mirror) {
            result += 100 * horizontal_index;
        }

        if let Some(vertical_index) = vertical_index(&mirror) {
            result += vertical_index;
        }
    }

    println!("Final result: {result}");
}
