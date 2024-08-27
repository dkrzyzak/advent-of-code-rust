use std::collections::HashSet;
use crate::{common::{direction::Direction, point::Point}, parse_input};

mod point;
use point::*;

pub fn task() {
    let lines = parse_input!();
    let cave = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut biggest_energized = 0;

    let last_col = cave[0].len() - 1;
    for row in 0..cave.len() {
        let result = traverse_cave(
            Point(row as isize, 0),
            Direction::East,
            &cave,
            &mut HashSet::new(),
            &mut HashSet::new(),
        );

        biggest_energized = biggest_energized.max(result);

        let result = traverse_cave(
            Point(row as isize, last_col as isize),
            Direction::West,
            &cave,
            &mut HashSet::new(),
            &mut HashSet::new(),
        );

        biggest_energized = biggest_energized.max(result);
    }

    let last_row = cave.len() - 1;
    for col in 0..cave[0].len() {
        let result = traverse_cave(
            Point(0, col as isize),
            Direction::South,
            &cave,
            &mut HashSet::new(),
            &mut HashSet::new(),
        );

        biggest_energized = biggest_energized.max(result);

        let result = traverse_cave(
            Point(last_row as isize, col as isize),
            Direction::North,
            &cave,
            &mut HashSet::new(),
            &mut HashSet::new(),
        );

        biggest_energized = biggest_energized.max(result);
    }

    println!("Biggest energized: {}", biggest_energized);
}

fn traverse_cave(
    point: Point,
    dir: Direction,
    cave: &Vec<Vec<char>>,
    visited_tiles: &mut HashSet<Point>,
    visited_splitters: &mut HashSet<Point>,
) -> u32 {
    if point.0 < 0
        || point.0 as usize >= cave.len()
        || point.1 < 0
        || point.1 as usize >= cave[0].len()
    {
        // we went outside the grid
        return 0;
    }

    let was_new = visited_tiles.insert(point.clone());
    let tile_value = if was_new { 1 } else { 0 };

    let current_tile = cave[point.0 as usize][point.1 as usize];
    match current_tile {
        '.' => {
            let next_point = point.next(&dir);

            return tile_value
                + traverse_cave(next_point, dir, cave, visited_tiles, visited_splitters);
        }
        '|' | '-' => {
            if visited_splitters.contains(&point) {
                // would result in an infinite loop
                return 0;
            }

            visited_splitters.insert(point.clone());

            if let Some(((point1, direction1), (point2, direction2))) =
                splitter(current_tile, &point, &dir)
            {
                return tile_value
                    + traverse_cave(point1, direction1, cave, visited_tiles, visited_splitters)
                    + traverse_cave(point2, direction2, cave, visited_tiles, visited_splitters);
            } else {
                let next_point = point.next(&dir);
                return tile_value
                    + traverse_cave(next_point, dir, cave, visited_tiles, visited_splitters);
            }
        }
        '\\' | '/' => {
            let (next_point, next_dir) = mirror(current_tile, &point, &dir);
            
            return tile_value
                + traverse_cave(next_point, next_dir, cave, visited_tiles, visited_splitters);
        }
        _ => {
            return 0;
        }
    }
}
