use std::{collections::HashSet, time::Instant};

pub fn task() {
    let mut input = String::from("cqjxxyzz");
    let time = Instant::now();

    loop {
        input = increment_password(&input);
        println!("Generated another: {}", &input);

        if validate_password(&input) {
            break;
        }
    }

    let elapsed = time.elapsed();
    println!("Elapsed: {:?}", elapsed);

    println!("found password: {input}");
}

fn increment_password(pass: &String) -> String {
    let mut new_pass: Vec<char> = pass.chars().collect();
    let mut index = pass.len() - 1;

    loop {
        if new_pass[index] == 'z' {
            new_pass[index] = 'a';
            index -= 1;
        } else {
            let next_char = (new_pass[index] as u8 + 1) as char;
            new_pass[index] = next_char;

            if next_char == 'i' || next_char == 'o' || next_char == 'l' {
                // increase again and reset all previous letters to "a"
                new_pass[index] = (next_char as u8 + 1) as char;
                let mut k = index + 1;
                while k != pass.len() {
                    new_pass[k] = 'a';
                    k += 1;
                }
            }

            break;
        }
    }

    new_pass.into_iter().collect()
}

fn validate_password(pass: &str) -> bool {
    if pass.contains('i') || pass.contains('o') || pass.contains('l') {
        return false;
    }

    let chars: Vec<char> = pass.chars().collect();
    let mut pairs: HashSet<char> = HashSet::new();
    let mut has_sequence = false;

    // test for sequence
    let mut i = 0;
    while i < chars.len() - 3 {
        if (chars[i + 1] as u8) == chars[i] as u8 + 1 && (chars[i + 2] as u8) == chars[i] as u8 + 2
        {
            has_sequence = true;
            break;
        }
        i += 1;
    }

    if !has_sequence {
        return false;
    }

    i = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pairs.insert(chars[i]);
            i += 2;
        } else {
            i += 1;
        }
    }

    return pairs.len() >= 2;
}
