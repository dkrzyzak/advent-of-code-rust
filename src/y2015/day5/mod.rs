use std::collections::HashMap;

use crate::read::read_input_file;

fn is_nice_1(s: &String) -> bool {
    let vowels: Vec<char> = "aeiou".chars().collect();
    let mut vowels_count = 0;
    let mut has_doubled_letter = false;

    let chars: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        let ch = chars[i];

        // counting vowels
        if vowels.contains(&ch) {
            vowels_count += 1;
        }

        if i > 0 {
            let previous = chars[i - 1];

            // verifying banned strings
            if (previous == 'a' && ch == 'b')
                || (previous == 'c' && ch == 'd')
                || (previous == 'p' && ch == 'q')
                || (previous == 'x' && ch == 'y')
            {
                return false;
            }

            // counting double letters
            if !has_doubled_letter && previous == ch {
                has_doubled_letter = true;
            }
        }
    }

    return vowels_count >= 3 && has_doubled_letter;
}

fn is_nice_2(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let length = s.len();

    let mut has_mirrored_pair = false;
    let mut pair_map: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..length {
        let ch = chars[i];

        if i < length - 1 {
            let pair_str = &s[i..i + 2]; // we need i and i+1, i+2 is the delimiter

            pair_map
                .entry(pair_str)
                .or_insert_with(Vec::new)
                .push((i, i + 1));

            if i > 0 {
                if !has_mirrored_pair && chars[i - 1] == chars[i + 1] {
                    has_mirrored_pair = true;
                }
            }
        }
    }
    // filter out the pairs that occurred only once
    let potential_pairs: Vec<(&&str, &Vec<(usize, usize)>)> = pair_map
        .iter()
        .filter(|(_, pairs_indices_vec)| pairs_indices_vec.len() > 1)
        .collect();

    if potential_pairs.len() < 1 {
        return false;
    }

    let mut actual_pairs = 0;

    // potential_pairs could be { "sw": Vec[(0, 1), (12, 14)], "hc": Vec[(1, 2), (2, 3)] }
    // we need to find how many of them are non-overlapping pairs. 
    // if there will be at least one non-overlapping pair, then we're good
    for (_, value) in potential_pairs.iter() {
        
        let mut actual_pair_count = 1;

        let mut was_overlapped = false;

        for i in 1..value.len() {
            let prev = if was_overlapped {
                value[i - 2]
            } else {
                value[i - 1]
            };

            let curr = value[i];

            if prev.1 != curr.0 {
                actual_pair_count += 1;
                was_overlapped = false;
            } else {
                was_overlapped = true;
            }
        }

        if actual_pair_count > 1 {
            actual_pairs += 1;
        }
    }

    return actual_pairs > 0 && has_mirrored_pair;
}

pub fn task() {
    let lines = read_input_file("y2015", "day5");

    let sum: i32 = lines
        .iter()
        .map(|line| if is_nice_2(line) { 1 } else { 0 })
        .sum();

    println!("Sum: {sum}");
}
