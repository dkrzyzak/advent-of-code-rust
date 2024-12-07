use std::{collections::VecDeque, time::Instant};

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let equations = extract_input(&lines);

    let now = Instant::now();

    let result = equations
        .into_iter()
        .map(|(result, numbers)| if evaluate_equation(result, numbers) > 0 { result } else { 0 })
        .sum::<u64>();

    println!("Result: {}", result);
    println!("Done in {:?}", now.elapsed());
}

pub fn extract_input(lines: &Vec<String>) -> Vec<(u64, VecDeque<u64>)> {
    lines
        .iter()
        .map(|line| {
            let mut split_colon = line.split(": ");
            let result = split_colon.next().unwrap();
            let numbers = split_colon.next().unwrap();

            let numbers_vec = numbers
                .split_whitespace()
                .map(|value| value.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>();

            let result_number = result.parse::<u64>().unwrap();

            (result_number, numbers_vec)
        })
        .collect::<Vec<_>>()
}

pub fn evaluate_equation(result: u64, mut numbers: VecDeque<u64>) -> u64 {
    if numbers.len() > 1 {
        let first = numbers.pop_front().unwrap();
        let second = numbers.pop_front().unwrap();

        let add_result = first + second;
        let mul_result = first * second;
        let con_result = format!("{first}{second}").parse::<u64>().unwrap();

        let mut updated_numbers_add = numbers.clone();
        updated_numbers_add.push_front(add_result);

        let mut updated_numbers_mul = numbers.clone();
        updated_numbers_mul.push_front(mul_result);

        // PART 2
        let mut updated_numbers_con = numbers.clone();
        updated_numbers_con.push_front(con_result);

        return evaluate_equation(result, updated_numbers_add)
            + evaluate_equation(result, updated_numbers_mul) 
            // PART 2:
            + evaluate_equation(result, updated_numbers_con);
    }

    return (numbers[0] == result) as u64;
}
