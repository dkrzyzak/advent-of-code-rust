use crate::{common::{direction::Direction, point::Point}, parse_input};

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let dig_plan = extract_dig_plan_2(&lines);
    let mut field = Vec::<Point>::new();

    dig_corners(&dig_plan, &mut field, Point(0, 0));

    let x = shoelace_formula(&field);
    println!("Filled size: {}", x);
}

pub fn dig_corners(dig_plan: &Vec<DigInstr>, field: &mut Vec<Point>, starting_point: Point) {
    let mut iter = dig_plan.iter();
    field.push(starting_point);

    let mut current_point = starting_point;

    while let Some(instr) = iter.next() {
        let DigInstr(dir, steps) = instr;

        let vector = match dir {
            Direction::East => Point(0, *steps as isize),
            Direction::West => Point(0, -(*steps as isize)),
            Direction::North => Point(-(*steps as isize), 0),
            Direction::South => Point(*steps as isize, 0),
        };
        
        current_point = current_point + vector;
        field.push(current_point);
    }
}


// FIRST OLD APPROACH

pub fn task_with_flood_fill() {
    let lines = parse_input!();
    let dig_plan = extract_dig_plan_1(&lines);
    let mut field = Vec::<Point>::new();
    dig_all(&dig_plan, &mut field, Point(0, 0));
    let filled = flood_fill(&field);

    println!("Filled: {}", filled.len());
}

pub fn dig_all(dig_plan: &Vec<DigInstr>, field: &mut Vec<Point>, starting_point: Point) {
    let mut iter = dig_plan.iter();
    let mut current_point = starting_point;

    while let Some(instr) = iter.next() {
        let DigInstr(dir, steps) = instr;

        for _ in 0..*steps {
            field.push(current_point);
            current_point = current_point.next(dir);
        }
    }
}