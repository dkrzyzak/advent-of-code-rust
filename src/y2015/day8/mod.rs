use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let mut combined_diff_1: usize = 0;
    let mut combined_diff_2: usize = 0;

    for line in lines {
        let (entire1, parsed) = parse_string(&line);
        combined_diff_1 += entire1 - parsed;

        let (entire2, encoded) = encode_string(&line);
        combined_diff_2 += encoded - entire2;

    }
    println!("Combined difference for part 1: {combined_diff_1}");
    println!("Combined difference for part 2: {combined_diff_2}");
}

fn parse_string(s: &str) -> (usize, usize) {
    let without_quotes = &s[1..s.len() - 1];
    let mut chars = without_quotes.chars().peekable();
    let mut parsed_length: usize = 0;

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                // possible escape instruction, we need to match upcoming symbols
                match chars.next() {
                    Some('x') => {
                        let h1 = chars.next().unwrap();
                        let h2 = chars.next().unwrap();
                        let hex_str = format!("{h1}{h2}");
                        if let Ok(final_value) = u8::from_str_radix(&hex_str, 16) {
                            parsed_length += 1;
                        }
                    }
                    Some('\\') => { parsed_length += 1 },
                    Some('"') => { parsed_length += 1 },
                    _ => { unreachable!("Wrong escape"); },
                }
            }
            _ => {
                parsed_length += 1;
            }
        }
    }

    return (s.len(), parsed_length);
}

fn encode_string(s: &str) -> (usize, usize) {
    let mut encoded = String::new();
    // opening quotes
    encoded.push('"');

    for ch in s.chars() {
        match ch {
            '\"' => {
                encoded.push('\\');
                encoded.push('\"');
            }
            '\\' => {
                encoded.push('\\');
                encoded.push('\\');
            }
            _ => { encoded.push(ch) }
        }
    }

    // closing quotes
    encoded.push('"');

    return (s.len(), encoded.len());
}