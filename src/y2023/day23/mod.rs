use std::collections::HashSet;

use crate::{common::point::Point, parse_input};

pub fn task() {
    let lines = parse_input!();
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = Point::new(0, 1);
    let end = Point::new((lines.len() - 1) as isize, (lines[0].len() - 2) as isize);

    let mut longest_path = 0;
    // let mut starting_path = HashSet::new();
    // starting_path.insert(start);

    println!("Start: {:?}, end: {:?}", start, end);

    next_move(start, &grid, &end, HashSet::new(), &mut longest_path);

    println!("Longest path: {}", longest_path - 1); // -1 because start point doesn't count
}

pub fn next_move(
    point: Point,
    grid: &Vec<Vec<char>>,
    end: &Point,
    mut path: HashSet<Point>,
    longest: &mut usize,
) {
    if !point.is_valid(grid.len() as isize, grid[0].len() as isize) {
        return; // we stepped out of the grid
    }

    let tile = grid[point.0 as usize][point.1 as usize];
    
    if tile == '#' {
        return; // we ended up in the forrest
    }

    if !path.insert(point) {
        return; // we've been there before
    }

    if point == *end {
        if path.len() > *longest {
            *longest = path.len();
        }

        return;
    }
    
    match tile {
        '.' => {
            next_move(point.east(), grid, end, path.clone(), longest);
            next_move(point.west(), grid, end, path.clone(), longest);
            next_move(point.north(), grid, end, path.clone(), longest);
            next_move(point.south(), grid, end, path.clone(), longest);
        },
        '>' => {
            next_move(point.east(), grid, end, path, longest);
        },
        '<' => {
            next_move(point.west(), grid, end, path, longest);
        },
        '^' => {
            next_move(point.north(), grid, end, path, longest);
        },
        'v' => {
            next_move(point.south(), grid, end, path, longest);
        },
        _ => unreachable!(),
    }

}
