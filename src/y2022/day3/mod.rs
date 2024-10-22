use std::collections::HashSet;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    
    // part_1(&lines);
    part_2(&lines);
}

pub fn part_1(lines: &Vec<String>) {
    let prios = lines.iter().map(|line| {
        let half = line.len() / 2;
        let chars = line.chars().collect::<Vec<_>>();

        let first = &chars[0..half];
        let second = &chars[half..];

        let repeated = first.iter().find(|ch| second.contains(ch)).unwrap();
        let prio = get_priority(repeated);

        prio as u32
    }).collect::<Vec<_>>();

    let sum = prios.iter().sum::<u32>();
    println!("Sum: {}", sum);
}

pub fn part_2(lines: &Vec<String>) {
    let mut elves = lines.iter();
    let mut prios = 0u32;

    while let (Some(first), Some(second), Some(third)) = (elves.next(), elves.next(), elves.next()) {
        let common12 = first.chars().filter(|ch| second.contains(*ch)).collect::<HashSet<_>>();
        let badge = common12.iter().find(|ch| third.contains(**ch)).unwrap();

        let prio = get_priority(badge);
        prios += prio as u32;
    }

    println!("Total prios: {}", prios);

}

pub fn get_priority(item: &char) -> u8 {
    let ascii_value = *item as u8;

    // uppercase:
    if ascii_value <= 90 {
        return ascii_value - 65 + 27;
    }

    return ascii_value - 96;
}
