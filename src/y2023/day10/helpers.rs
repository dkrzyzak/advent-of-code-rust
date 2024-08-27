use std::collections::HashMap;
use crate::common::point::Point;

#[derive(Debug)]
struct Neighbour {
    vector: Point,
    allowed_symbols: Vec<char>,
}

pub fn find_start(grid: &Vec<Vec<char>>) -> Point {
    let mut s_index = Point::new(0, 0);

    for (y_index, inner_vec) in grid.iter().enumerate() {
        for (x_index, &item) in inner_vec.iter().enumerate() {
            if item == 'S' {
                s_index = Point::new(x_index as isize, y_index as isize);
                break;
            }
        }
    }

    s_index
}

pub fn part_1(grid: &Vec<Vec<char>>, board_size: &(usize, usize)) -> Vec<Point> {
    
    println!("Board size: {:?}", board_size);

    #[rustfmt::skip]
    let map = HashMap::from([
    ('|', vec![
    Neighbour { vector: Point::new(0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S'] },
    Neighbour { vector: Point::new(0, -1), allowed_symbols: vec!['|', 'F', '7', 'S']},
    ]),

    ('-', vec![
    Neighbour { vector: Point::new(1, 0), allowed_symbols: vec!['-', 'J', '7', 'S'] },
    Neighbour { vector: Point::new(-1, 0), allowed_symbols: vec!['-', 'L', 'F', 'S']},
    ]),

    ('L', vec![
    Neighbour { vector: Point::new(0, -1), allowed_symbols: vec!['|', 'F', '7'] },
    Neighbour { vector: Point::new(1, 0), allowed_symbols: vec!['-', 'J', '7', 'S']},
    ]),

    ('J', vec![
    Neighbour { vector: Point::new(0, -1), allowed_symbols: vec!['|', 'F', '7', 'S'] },
    Neighbour { vector: Point::new(-1, 0), allowed_symbols: vec!['-', 'F', 'L', 'S']},
    ]),

    ('F', vec![
    Neighbour { vector: Point::new(1, 0), allowed_symbols: vec!['-', '7', 'J', 'S'] },
    Neighbour { vector: Point::new(0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S']},
    ]),

    ('7', vec![
    Neighbour { vector: Point::new(0, 1), allowed_symbols: vec!['|', 'L', 'J', 'S']},
    Neighbour { vector: Point::new(-1, 0), allowed_symbols: vec!['-', 'F', 'L', 'S'] },
    ]),

    ('S', vec![
    Neighbour { vector: Point::new(1, 0), allowed_symbols: vec!['-', 'J', '7'] },
    Neighbour { vector: Point::new(0, 1), allowed_symbols: vec!['|', 'L', 'J']},
    Neighbour { vector: Point::new(-1, 0), allowed_symbols: vec!['-', 'F', 'L']},
    Neighbour { vector: Point::new(0, -1), allowed_symbols: vec!['|', 'F', '7']},
    ]),
    ]);

    let start = find_start(&grid);

    let mut visited: Vec<Point> = Vec::new();
    let mut current_idx = start;
    println!("Start: {:?}", start);

    loop {
        let current_symbol = grid[current_idx.1 as usize][current_idx.0 as usize];

        let neighbours = map.get(&current_symbol).unwrap();

        'find: for neighbour in neighbours {
            let new_point = current_idx + neighbour.vector;

            if new_point.0 < 0
                || new_point.1 < 0
                || new_point.0 >= board_size.0 as isize
                || new_point.1 >= board_size.1 as isize
            {
                continue;
            }

            if visited.contains(&new_point) {
                continue;
            }

            let new_symbol = grid[new_point.1 as usize][new_point.0 as usize];

            for allowed_symbol in &neighbour.allowed_symbols {
                if new_symbol == *allowed_symbol {
                    current_idx = new_point;
                    visited.push(current_idx);

                    break 'find;
                }
            }
        }

        if grid[current_idx.1 as usize][current_idx.0 as usize] == 'S' {
            break;
        }
    }

    let distance = visited.len();
    println!("distance = {}, middle point: {}", distance, distance / 2);

    visited
}
