use regex::Regex;
use crate::{common::{direction::Direction, point::Point}, parse_captures, parse_input};

#[derive(Debug)]
pub struct DigInstr(Direction, u8, String);

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let dig_plan = extract_dig_plan(&lines);
    let mut field = Vec::<Point>::new();

    dig(&dig_plan, &mut field, Point(0, 0));
    
    // field.print_by_line();

    let filled = flood_fill(&field);
    // let x = shoelace_formula(&field);
    println!("Filled size: {}", filled.len());
}

pub fn dig(dig_plan: &Vec<DigInstr>, field: &mut Vec<Point>, starting_point: Point) {
    let mut iter = dig_plan.iter();
    let mut current_point = starting_point;

    while let Some(instr) = iter.next() {
        let DigInstr(dir, steps, _) = instr;

        for _ in 0..*steps {
            field.push(current_point);
            current_point = current_point.next(dir);
        }
    }
}

fn extract_dig_plan(lines: &Vec<String>) -> Vec<DigInstr> {
    let reg = Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap();

    lines.iter().map(|line| {
        let (dir, steps, hex): (char, u8, String) = parse_captures!(&reg, line, 1 char, 2 u8, 3 String);

        let mapped_dir = match dir {
            'U' => Direction::North,
            'D' => Direction::South,
            'L' => Direction::West,
            'R' => Direction::East,
            _ => { panic!("Invalid direction") }
        };

        DigInstr(mapped_dir, steps, hex)
    }).collect::<Vec<_>>()
}
