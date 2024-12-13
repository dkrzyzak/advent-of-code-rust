use std::time::Instant;

use crate::parse_input;

mod arcade;
use arcade::*;

pub fn task() {
    let lines = parse_input!();
    let machines = extract_machines(&lines);

    let mut min_tokens_spent = 0;

    let now = Instant::now();

    for machine in machines.iter() {
        if let Some(min_cost) = solve_part_2(machine) {
            min_tokens_spent += min_cost;
        }
    }

    println!("Minimum tokens spent: {}", min_tokens_spent);
    println!("calculated in {:?}", now.elapsed());

    // 33842 is too low
}

pub fn solve_part_1(machine: &Machine) -> Option<u64> {
    let max_clicks = 100;
    let mut cheapest: Option<u64> = None;

    for a in 0..max_clicks {
        if machine.prize_x < machine.ax * a {
            // a is too big already
            continue;
        }

        let b_from_x = (machine.prize_x - machine.ax * a) as f64 / machine.bx as f64;

        // skip if b is not an integer
        if b_from_x != b_from_x.floor() || b_from_x < 0.0 || b_from_x > max_clicks as f64 {
            continue;
        }

        let b = b_from_x as u64;

        let y_result = machine.ay * a + machine.by * b;

        if y_result == machine.prize_y {
            let cost = a * 3 + b;
            if cost < cheapest.unwrap_or(u64::MAX) {
                cheapest = Some(cost);
            }
        }
    }

    cheapest
}

pub fn solve_part_2(machine: &Machine) -> Option<u64> {
    const OFFSET: u64 = 10_000_000_000_000;

    // calculated on paper xd
    let numerator = ((machine.by * (machine.prize_x + OFFSET)) as i64 - (machine.bx * (machine.prize_y + OFFSET)) as i64) as f64;
    let denominator = ((machine.ax * machine.by) as i64 - (machine.ay * machine.bx) as i64) as f64;

    let a = numerator / denominator;

    if a != a.floor() || a < 0.0 {
        return None;
    }

    // also calculated on paper
    let b = ((machine.prize_x as i64 + OFFSET as i64 - a as i64 * machine.ax as i64) as f64) / machine.bx as f64;

    if b != b.floor() || b < 0.0 {
        return None;
    }

    let cost = 3 * a as u64 + b as u64;

    return Some(cost);
}