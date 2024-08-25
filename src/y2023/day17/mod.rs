use crate::parse_input;

mod point;
mod dijkstra;
use dijkstra::*;

pub fn task() {
    let lines = parse_input!();
    let map = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();



    match shortest_path(&map) {
        Some(cost) => println!("The cost of the cheapest path is: {}", cost),
        None => println!("No path found."),
    }
}
