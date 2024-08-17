use crate::point::Point;
use std::collections::HashMap;

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
                s_index = Point::new(x_index as i32, y_index as i32);
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
        let current_symbol = grid[current_idx.y as usize][current_idx.x as usize];

        let neighbours = map.get(&current_symbol).unwrap();

        'find: for neighbour in neighbours {
            let new_point = current_idx + neighbour.vector;

            if new_point.x < 0
                || new_point.y < 0
                || new_point.x >= board_size.0 as i32
                || new_point.y >= board_size.1 as i32
            {
                continue;
            }

            if visited.contains(&new_point) {
                continue;
            }

            let new_symbol = grid[new_point.y as usize][new_point.x as usize];

            for allowed_symbol in &neighbour.allowed_symbols {
                if new_symbol == *allowed_symbol {
                    current_idx = new_point;
                    visited.push(current_idx);

                    break 'find;
                }
            }
        }

        if grid[current_idx.y as usize][current_idx.x as usize] == 'S' {
            break;
        }
    }

    let distance = visited.len();
    println!("distance = {}, middle point: {}", distance, distance / 2);

    visited
}
