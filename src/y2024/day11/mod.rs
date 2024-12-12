use std::collections::HashMap;

use crate::parse_input;

type Cache = HashMap<(usize, usize), usize>;

pub fn task() {
    let lines = parse_input!();
    let input = lines.first().unwrap();
    let data = input
        .split_whitespace()
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let iters = 75;
    let mut cache: Cache = HashMap::new();

    let sum = data
        .iter()
        .map(|val| blink_recursive(*val, iters, &mut cache))
        .sum::<usize>();

    println!("Stones total: {}", sum);
}

pub fn blink_recursive(number: usize, iters_left: usize, cache: &mut Cache) -> usize {
    if let Some(&result) = cache.get(&(number, iters_left)) {
        return result;
    }

    let result = if iters_left == 0 {
        1
    } else if number == 0 {
        blink_recursive(1, iters_left - 1, cache)
    } else if has_even_digits(number) {
        let (left, right) = split_number(number);

        blink_recursive(left, iters_left - 1, cache) + blink_recursive(right, iters_left - 1, cache)
    } else {
        blink_recursive(number * 2024, iters_left - 1, cache)
    };

    cache.insert((number, iters_left), result);
    result
}

pub fn has_even_digits(number: usize) -> bool {
    number.to_string().len() % 2 == 0
}

pub fn split_number(number: usize) -> (usize, usize) {
    let s = number.to_string();
    let mid = s.len() / 2;

    let (first, second) = s.split_at(mid);

    let first_parsed = first.parse::<usize>().unwrap();
    let second_parsed = second.parse::<usize>().unwrap();

    (first_parsed, second_parsed)
}
