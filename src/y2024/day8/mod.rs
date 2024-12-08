use std::collections::{HashMap, HashSet};

use crate::{
    common::{grid::Grid, point::Point},
    parse_input,
};

type Antennas = HashMap<char, Vec<Point>>;

pub fn task() {
    let lines = parse_input!();
    let (grid, antennas_map) = extract_antennas(&lines);

    part1(&grid, &antennas_map);

    part2(&grid, &antennas_map);
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

pub fn part1(grid: &Grid, antennas_map: &Antennas) {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (symbol, antennas) in antennas_map.iter() {
        let pairs: Vec<(&Point, &Point)> = antennas
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| antennas.iter().skip(i + 1).map(move |p2| (p1, p2)))
            .collect();

        for &(a1, a2) in pairs.iter() {
            let d_vector = a2 - a1;

            let antinode_backward = *a1 - d_vector;
            let antinode_forward = *a2 + d_vector;

            if grid.contains_point(&antinode_backward) {
                antinodes.insert(antinode_backward);
            }

            if grid.contains_point(&antinode_forward) {
                antinodes.insert(antinode_forward);
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
}

pub fn part2(grid: &Grid, antennas_map: &Antennas) {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (symbol, antennas) in antennas_map.iter() {
        let pairs: Vec<(&Point, &Point)> = antennas
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| antennas.iter().skip(i + 1).map(move |p2| (p1, p2)))
            .collect();

        for &(a1, a2) in pairs.iter() {
            let d_vector = a2 - a1;

            let mut antinodes_backward = *a1;
            while grid.contains_point(&antinodes_backward) {
                antinodes.insert(antinodes_backward);
                antinodes_backward = antinodes_backward - d_vector;
            }

            let mut antinodes_forward = *a2;
            while grid.contains_point(&antinodes_forward) {
                antinodes.insert(antinodes_forward);
                antinodes_forward = antinodes_forward + d_vector;
            }
        }
    }

    println!("Part 2: {}", antinodes.len());
}
