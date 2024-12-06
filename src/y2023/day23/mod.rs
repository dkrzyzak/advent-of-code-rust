use std::{
    cmp,
    collections::{HashSet, VecDeque},
    time::Instant,
};

use crate::{
    common::{grid::Grid, point::Point},
    parse_input,
};

pub fn task() {
    let lines = parse_input!();
    let grid = Grid::from_vec(&lines);

    traverse_dfs(&grid);
}

pub fn traverse_dfs(grid: &Grid) {
    let start = Point::new(0, 1);
    let end = Point::new(grid.irows - 1, grid.icols - 2);
    let mut longest_path = 0;

    // (point, path)
    let mut queue = VecDeque::<(Point, HashSet<Point>)>::new();
    queue.push_back((start, HashSet::new()));

    let now = Instant::now();

    while let Some((point, mut path)) = queue.pop_back() {
        path.insert(point);
        let tile = grid.iget(point.0, point.1).unwrap();

        let path_len = path.len();

        if point == end {
            longest_path = cmp::max(longest_path, path_len);
            continue;
        }

        match tile {
            '.' | '>' | '<' | '^' | 'v' => {
                vec![point.east(), point.west(), point.north(), point.south()]
                    .iter()
                    .for_each(|&point| {
                        if should_explore(&point, grid, &path) {
                            queue.push_back((point, path.clone()))
                        }
                    });
            }
            _ => unreachable!(),
        }
    }

    println!("Done in: {:?}", now.elapsed());

    println!("Longest path: {}", longest_path - 1); // -1 because start point doesn't count
}

pub fn should_explore(point: &Point, grid: &Grid, path: &HashSet<Point>) -> bool {
    if !grid.contains(point) {
        return false; // we stepped out of the grid
    }

    let tile = grid.iget(point.0, point.1).unwrap();

    if tile == '#' {
        return false; // we ended up in the forrest
    }

    if path.contains(point) {
        return false; // we've been there before
    }

    true
}

// 4710 is too low
