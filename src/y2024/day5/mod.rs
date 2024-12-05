use crate::parse_input;

mod extract;
use extract::*;

pub fn task() {
    let lines = parse_input!();
    let (rules, mut updates) = extract_input(&lines);

    // let total = part1(&rules, &updates);
    // println!("Part 1: {}", total);

    let total2 = part2(&rules, &mut updates);
    println!("Part 2: {}", total2);
}

pub fn part1(rules: &Vec<Rule>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut total = 0;

    for update in updates.iter() {
        if let Some(_) = get_first_broken_rule(rules, update) {
            continue;
        }

        total += get_middle_element(update);
    }

    total
}

pub fn part2(rules: &Vec<Rule>, updates: &mut Vec<Vec<u32>>) -> u32 {
    let mut total = 0;

    for update in updates.iter_mut() {
        if let Some(broken_rule) = get_first_broken_rule(rules, update) {
            fix_update(rules, update, broken_rule);

            total += get_middle_element(update);
        } else {
            continue;
        }
    }

    total
}

pub fn get_first_broken_rule<'a>(rules: &'a Vec<Rule>, update: &Vec<u32>) -> Option<&'a Rule> {
    for rule in rules.iter() {
        if !rule.is_valid_update(update) {
            return Some(rule);
        }
    }

    return None;
}

pub fn fix_update(rules: &Vec<Rule>, update: &mut Vec<u32>, first_broken_rule: &Rule) {
    // we know now, that first appears after the second
    // we should move second just after the first
    let &Rule(first, second) = first_broken_rule;
    let first_index = update.iter().position(|&el| el == first).unwrap();
    let second_index = update.iter().position(|&el| el == second).unwrap();

    update.remove(second_index);
    update.insert(first_index, second);

    if let Some(next_broken_rule) = get_first_broken_rule(rules, update) {
        fix_update(rules, update, next_broken_rule);
    }
}

pub fn get_middle_element(vec: &Vec<u32>) -> u32 {
    vec[vec.len() / 2]
}
