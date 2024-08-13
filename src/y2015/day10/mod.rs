pub fn task() {
    let mut input = String::from("3113322113");
    let mut iterations = 50;
    while iterations > 0 {
        input = look_and_say(input);
        iterations -= 1;
    }
    println!("Result len: {}", input.len());
}

fn look_and_say(s: String) -> String {
    let digits: Vec<u32> = s.chars().map(|ch| ch.to_digit(10).unwrap_or(0)).collect();

    let mut result = String::new();
    let mut digits_iter = digits.iter().peekable();
    let mut current_digit = digits_iter.next().unwrap();
    let mut current_digit_count = 1;

    while let Some(next_digit) = digits_iter.next() {
        if next_digit != current_digit {
            result.push_str(current_digit_count.to_string().as_str());
            result.push_str(current_digit.to_string().as_str());
            current_digit = next_digit;
            current_digit_count = 1;
        } else {
            current_digit_count += 1;
        }
    }

    result.push_str(current_digit_count.to_string().as_str());
    result.push_str(current_digit.to_string().as_str());

    return result;
}
