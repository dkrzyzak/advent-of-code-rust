use std::collections::{HashMap, HashSet};

use crate::{
    common::{grid::Grid, point::Point},
    parse_input,
};

type Antennas = HashMap<char, Vec<Point>>;

pub fn task() {
    let lines = parse_input!();
    let (grid, antennas_map) = extract_antennas(&lines);

    let mut antinodes: HashSet<Point> = HashSet::new();
    for (symbol, antennas) in antennas_map.iter() {
        let pairs: Vec<(&Point, &Point)> = antennas
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| antennas.iter().skip(i + 1).map(move |p2| (p1, p2)))
            .collect();

        for &(a1, a2) in pairs.iter() {
            let d_vector = a2 - a1;
            // println!("{:?} - {:?}, vector: {:?}", a1, a2, d_vector);
            let antinode_1 = *a1 - d_vector;
            let antinode_2 = *a2 + d_vector;

            if let Some(symbol_at_node) = grid.get_point(&antinode_1) {
                antinodes.insert(antinode_1);
            }

            if let Some(symbol_at_node) = grid.get_point(&antinode_2) {
                antinodes.insert(antinode_2);
            }
        }
    }

    println!("Total: {}", antinodes.len());
}

pub fn extract_antennas(lines: &Vec<String>) -> (Grid, Antennas) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut antennas: Antennas = HashMap::new();

    let rows = lines.len();
    let cols = lines[0].len();
    for row in 0..rows {
        let symbols = lines[row].chars().collect::<Vec<_>>();
        let mut col_vec = Vec::new();

        for col in 0..cols {
            let item = symbols[col];
            col_vec.push(item);
            if item != '.' {
                let point = Point(row as isize, col as isize);
                antennas.entry(item).or_insert_with(Vec::new).push(point);
            }
        }

        grid.push(col_vec);
    }

    (Grid::initialize(grid), antennas)
}
