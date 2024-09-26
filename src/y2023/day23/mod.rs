use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

use crate::{common::point::Point, parse_input};

pub fn task() {
    let lines = parse_input!();
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    traverse_bfs(&grid);
}

pub fn traverse_bfs(grid: &Vec<Vec<char>>) {
    let start = Point::new(0, 1);
    let end = Point::new((grid.len() - 1) as isize, (grid[0].len() - 2) as isize);
    let mut longest_path = 0;

    let mut queue = VecDeque::<(Point, HashSet<Point>)>::new();
    queue.push_back((start, HashSet::new()));

    while let Some((point, mut path)) = queue.pop_front() {
        if !point.is_valid(grid.len() as isize, grid[0].len() as isize) {
            continue; // we stepped out of the grid
        }

        let tile = grid[point.0 as usize][point.1 as usize];

        if tile == '#' {
            continue; // we ended up in the forrest
        }

        if !path.insert(point) {
            continue; // we've been there before
        }

        if point == end {
            longest_path = cmp::max(longest_path, path.len());
            continue;
        }

        match tile {
            '.' => {
                queue.push_back((point.east(), path.clone()));
                queue.push_back((point.west(), path.clone()));
                queue.push_back((point.north(), path.clone()));
                queue.push_back((point.south(), path.clone()));
            }
            '>' => queue.push_back((point.east(), path.clone())),
            '<' => queue.push_back((point.west(), path.clone())),
            '^' => queue.push_back((point.north(), path.clone())),
            'v' => queue.push_back((point.south(), path.clone())),
            _ => unreachable!(),
        }
    }

    println!("Longest path: {}", longest_path - 1); // -1 because start point doesn't count
}
