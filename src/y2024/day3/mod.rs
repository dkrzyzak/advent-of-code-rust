use regex::Regex;
use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let memory = lines.join("");

    let mut total = 0u32;
    let mut enabled = true;

    let reg = Regex::new(r"mul\((\d+),(\d+)\)|do(?:n't)?\(\)").unwrap();

    for captures in reg.captures_iter(&memory) {
        let captured = captures.get(0).unwrap().as_str();

        match captured {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let a = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let b = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    total += a * b;
                }
            }
        }
    }

    println!("Total: {}", total);
}
