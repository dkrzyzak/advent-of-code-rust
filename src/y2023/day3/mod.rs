use std::collections::HashMap;

use crate::{common::point::Point, parse_input};

const SYMBOLS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '/', '-', '=', '+'];

const NEIGHBOURS: &[Point] = &[
    Point(-1, -1),
    Point(-1, 0),
    Point(-1, 1),
    Point(0, -1),
    Point(0, 1),
    Point(1, -1),
    Point(1, 0),
    Point(1, 1),
];

#[derive(Debug)]
struct Number {
    value: u32,
    asterisk_position: Point,
}

fn get_value<T>(v: &Vec<Vec<T>>, row: isize, col: isize) -> Option<&T> {
    if col < 0 || row < 0 || col > 140 || row > 140 {
        return None;
    }

    v.get(row as usize)?.get(col as usize)
}

pub fn task1() {
    let lines: Vec<Vec<char>> = parse_input!()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;

    let mut reading_number = false;
    let mut current_num = String::new();
    let mut starting_y = 0;

    for (row, line) in lines.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                if !reading_number {
                    reading_number = true;
                    starting_y = col;
                }

                current_num.push(ch);
            } else {
                if reading_number {
                    let nr_len = current_num.len();
                    let parsed = current_num.parse::<u32>().unwrap_or_default();

                    current_num.clear();

                    'finding_adjacent_symbol: for i in 0..nr_len {
                        let y = starting_y + i;
                        for neighbour in NEIGHBOURS.iter() {
                            let nx = row as isize + neighbour.0;
                            let ny = y as isize + neighbour.1;

                            if let Some(adj_char) = get_value(&lines, nx, ny) {
                                if SYMBOLS.contains(adj_char) {
                                    sum += parsed;
                                    break 'finding_adjacent_symbol;
                                }
                            }
                        }
                    }

                    reading_number = false;
                }
            }
        }
    }

    println!("SUM: {sum}")
}

pub fn task2() {
    let lines: Vec<Vec<char>> = parse_input!()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut numbers_found: Vec<Number> = Vec::new();
    let mut reading_number = false;
    let mut current_num = String::new();
    let mut starting_y = 0;

    for (row, line) in lines.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch.is_numeric() {
                if !reading_number {
                    reading_number = true;
                    starting_y = col;
                }

                current_num.push(ch);
            } else {
                if reading_number {
                    let nr_len = current_num.len();
                    let parsed = current_num.parse::<u32>().unwrap_or_default();

                    current_num.clear();

                    'finding_adjacent_symbol: for i in 0..nr_len {
                        let y = starting_y + i;
                        for neighbour in NEIGHBOURS.iter() {
                            let nx = row as isize + neighbour.0;
                            let ny = y as isize + neighbour.1;

                            if let Some(adj_char) = get_value(&lines, nx, ny) {
                                if *adj_char == '*' {
                                    let coords = Point(nx, ny);
                                    let num = Number {
                                        value: parsed,
                                        asterisk_position: coords,
                                    };
                                    numbers_found.push(num);

                                    break 'finding_adjacent_symbol;
                                }
                            }
                        }
                    }

                    reading_number = false;
                }
            }
        }
    }

    let mut map_of_asterisks: HashMap<Point, Vec<u32>> = HashMap::new();

    for num in numbers_found.iter() {
        map_of_asterisks
            .entry(num.asterisk_position)
            .or_insert(Vec::new())
            .push(num.value);
    }

    let mut sum = 0;

    for (_, value) in map_of_asterisks.iter() {
        if value.len() == 2 {
            sum += value[0] * value[1];
        }
    }

    println!("SUM: {sum}");
}
