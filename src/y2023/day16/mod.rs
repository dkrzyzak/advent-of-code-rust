use std::collections::HashSet;

use crate::parse_input;

mod point;
use point::*;

pub fn task() {
    let lines = parse_input!();
    let cave = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited_tiles = HashSet::<Point>::new();
    let mut visited_splitters = HashSet::<Point>::new();
    let starting_point = Point { row: 0, col: 0 };
    let starting_direction = Direction::East;

    let total = traverse_cave(
        starting_point,
        starting_direction,
        &cave,
        &mut visited_tiles,
        &mut visited_splitters,
    );

    println!("All visited tiles: {}", total);
    // println!("Visited tiles: {:?}", visited_tiles);
    // println!("All visited splitters: {}: {:?}", visited_splitters.len(), visited_splitters);
}

fn traverse_cave(
    current_point: Point,
    direction: Direction,
    cave: &Vec<Vec<char>>,
    visited_tiles: &mut HashSet<Point>,
    visited_splitters: &mut HashSet<Point>,
) -> u32 {
    if current_point.row < 0
        || current_point.row as usize >= cave.len()
        || current_point.col < 0
        || current_point.col as usize >= cave[0].len()
    {
        // we went outside the grid
        return 0;
    }

    let was_new = visited_tiles.insert(current_point.clone());
    let tile_value = if was_new { 1 } else { 0 };

    let current_tile = cave[current_point.row as usize][current_point.col as usize];
    match current_tile {
        '.' => {
            let next_point = current_point.next(&direction);

            return tile_value + traverse_cave(
                next_point,
                direction,
                cave,
                visited_tiles,
                visited_splitters,
            );
        }
        '|' | '-' => {
            if visited_splitters.contains(&current_point) {
                // would result in an infinite loop
                return 0;
            }

            visited_splitters.insert(current_point.clone());

            if let Some(((point1, direction1), (point2, direction2))) =
                splitter(current_tile, &current_point, &direction)
            {
                return tile_value + traverse_cave(point1, direction1, cave, visited_tiles, visited_splitters)
                    + traverse_cave(point2, direction2, cave, visited_tiles, visited_splitters);
            } else {
                let next_point = current_point.next(&direction);
                return tile_value + traverse_cave(
                    next_point,
                    direction,
                    cave,
                    visited_tiles,
                    visited_splitters,
                );
            }
        }
        '\\' | '/' => {
            let (next_point, next_dir) = mirror(current_tile, &current_point, &direction);
            return tile_value + traverse_cave(next_point, next_dir, cave, visited_tiles, visited_splitters);
        }
        _ => {
            return 0;
        }
    }
}
