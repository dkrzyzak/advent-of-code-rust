use std::collections::{HashSet, VecDeque};
use regex::Regex;
use crate::{common::{direction::Direction, point::Point}, parse_captures};

#[derive(Debug)]
pub struct DigInstr(pub Direction, pub u64);

impl Point {
    pub fn min_x(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.0).min().unwrap()
    }

    pub fn min_y(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.1).min().unwrap()
    }

    pub fn max_x(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.0).max().unwrap()
    }

    pub fn max_y(points: &Vec<Point>) -> isize {
        points.iter().map(|p| p.1).max().unwrap()
    }

    pub fn bounding_box(points: &Vec<Point>) -> (isize, isize, isize, isize) {
        let min_x = Self::min_x(points);
        let max_x = Self::max_x(points);
        let min_y = Self::min_y(points);
        let max_y = Self::max_y(points);

        (min_x, min_y, max_x, max_y)
    }
}

pub fn extract_dig_plan_1(lines: &Vec<String>) -> Vec<DigInstr> {
    let reg = Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap();

    lines.iter().map(|line| {
        let (dir, steps, hex): (char, u64, String) = parse_captures!(&reg, line, 1 char, 2 u64, 3 String);

        let mapped_dir = match dir {
            'U' => Direction::North,
            'D' => Direction::South,
            'L' => Direction::West,
            'R' => Direction::East,
            _ => unreachable!("Invalid direction")
        };

        DigInstr(mapped_dir, steps)
    }).collect::<Vec<_>>()
}

pub fn extract_dig_plan_2(lines: &Vec<String>) -> Vec<DigInstr> {
    let reg = Regex::new(r"(\w) (\d+) \(#(\w+)\)").unwrap();

    lines.iter().map(|line| {
        let (dir, steps, hex): (char, u8, String) = parse_captures!(&reg, line, 1 char, 2 u8, 3 String);

        let (first_slice, second_slice) = hex.split_at(5);
        let parsed_steps = u64::from_str_radix(first_slice, 16).unwrap();

        let parsed_dir = match second_slice {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => unreachable!("Invalid direction")
        };

        // println!("nosplit: {}, first: {}, second: {:?}", hex, parsed_steps, parsed_dir);

        DigInstr(parsed_dir, parsed_steps)
    }).collect::<Vec<_>>()
}

pub fn shoelace_formula(points: &Vec<Point>) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let distance = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
                let new_perimeter = perimeter + distance;
                let new_area = sum + (p1.1 * p2.0) - (p1.0 * p2.1);

                (new_area, new_perimeter)
            });

    area.abs() / 2 + (perimeter / 2) + 1
}

pub fn flood_fill(perimeter: &Vec<Point>) -> Vec<Point> {
    let (min_x, min_y, max_x, max_y) = Point::bounding_box(perimeter);
    println!("Bounding box: ({}, {}) x ({}, {})", min_x, min_y, max_x, max_y);

    let mut result = perimeter.clone();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // start flood-fill from the top-left corner of the bounding box
    queue.push_back(Point(1, 1));

    while let Some(point) = queue.pop_front() {
        if point.0 < min_x || point.0 > max_x || point.1 < min_y || point.1 > max_y {
            continue;
        }

        if visited.contains(&point) || perimeter.contains(&point) {
            continue;
        }

        visited.insert(point);
        result.push(point);

        queue.push_back(point.north());
        queue.push_back(point.south());
        queue.push_back(point.west());
        queue.push_back(point.east());
    }

    result
}
