use std::collections::HashSet;

use crate::parse_input;

mod point;
use point::*;

pub fn task() {
    let lines = parse_input!();
    let map = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cheapest_route = u32::MAX;

    route_recursive(Point { row: 0, col: 0 }, Direction::East, 0, HashSet::new(), None, &map, 1, &mut cheapest_route);

    // println!("{:?}", map);
    println!("Cheapest route: {}", cheapest_route);
}

pub fn route_recursive(
    point: Point,
    dir: Direction,
    route_cost: u32,
    visited_points: HashSet<Point>,
    last_turn: Option<bool>,
    map: &Vec<Vec<u32>>,
    direction_count: u8,
    cheapest_route: &mut u32,
) {
    if point.row < 0
        || point.row as usize >= map.len()
        || point.col < 0
        || point.col as usize >= map[0].len()
    {
        // we went outside the grid
        return;
    }
    
    // if direction_count > 3 {
    //     // more than 3 in one direction
    //     return;
    // }

    if route_cost > *cheapest_route {
        // expensive, ineffective route
        return;
    }

    if visited_points.contains(&point) {
        // been there already
        return;
    }

    // println!("Past the return conditions. Point: {:?}", point);


    let block_cost = map[point.row as usize][point.col as usize];
    let acc_route_cost = route_cost + block_cost;
    let mut new_visited_points = visited_points.clone();
    new_visited_points.insert(point.clone());

    if point.row == map.len() as isize - 1  && point.col == map[0].len() as isize - 1 {
        if acc_route_cost < *cheapest_route {
            println!("New cheapest route: {}", acc_route_cost);
            *cheapest_route = acc_route_cost;
        }
        return;
    }

    if direction_count < 3 {
        let point_next = point.next(&dir);
        route_recursive(point_next, dir.clone(), acc_route_cost, new_visited_points.clone(), None, map, direction_count + 1, cheapest_route);
    }

    if let Some(turn) = last_turn {
        if turn {
            let (point_r, dir_r) = point.turn_right(&dir);
            route_recursive(point_r, dir_r, acc_route_cost, new_visited_points.clone(), Some(false), map, 1, cheapest_route);
        } else {
            let (point_l, dir_l) = point.turn_left(&dir);
            route_recursive(point_l, dir_l, acc_route_cost, new_visited_points.clone(), Some(true), map, 1, cheapest_route);
        }
    } else {
        let (point_r, dir_r) = point.turn_right(&dir);
        route_recursive(point_r, dir_r, acc_route_cost, new_visited_points.clone(), Some(false), map, 1, cheapest_route);

        let (point_l, dir_l) = point.turn_left(&dir);
        route_recursive(point_l, dir_l, acc_route_cost, new_visited_points.clone(), Some(true), map, 1, cheapest_route);
    }

}
