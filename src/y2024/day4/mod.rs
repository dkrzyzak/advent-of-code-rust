use crate::{common::grid::Grid, parse_input};

pub fn task() {
    let lines = parse_input!();
    let grid = Grid::from_vec(&lines);

    let all_xmas = part1(&grid);
    println!("All XMAS: {}", all_xmas);

    let all_x_mas = part2(&grid);
    println!("All X-shaped MAS: {}", all_x_mas);
}

pub fn part1(grid: &Grid) -> u32 {
    let word_variants: Vec<Vec<(isize, isize, char)>> = vec![
        vec![(0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S')], // horizontal forwards
        vec![(0, -1, 'M'), (0, -2, 'A'), (0, -3, 'S')], // horizontal backwards
        vec![(1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S')], // vertical forwards
        vec![(-1, 0, 'M'), (-2, 0, 'A'), (-3, 0, 'S')], // vertical backwards
        vec![(1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S')], // diagonal down forwards
        vec![(-1, 1, 'M'), (-2, 2, 'A'), (-3, 3, 'S')], // diagonal up forwards
        vec![(1, -1, 'M'), (2, -2, 'A'), (3, -3, 'S')], // diagonal down backwards
        vec![(-1, -1, 'M'), (-2, -2, 'A'), (-3, -3, 'S')], // diagonal up backwards
    ];

    let mut all_xmas = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col) != 'X' {
                continue;
            }

            'variant: for variant in word_variants.iter() {
                for &(row_vector, col_vector, expected_char) in variant.iter() {
                    let next_row = row as isize + row_vector;
                    let next_col = col as isize + col_vector;

                    if let Some(next_char) = grid.iget(next_row, next_col) {
                        if next_char != expected_char {
                            continue 'variant;
                        }
                    } else {
                        continue 'variant;
                    }
                }

                all_xmas += 1;
            }
        }
    }

    all_xmas
}

pub fn part2(grid: &Grid) -> u32 {
    // checking the X starting from A in the middle -> we need to provide vectors form M and S
    let word_variants: Vec<Vec<(isize, isize, char)>> = vec![
        vec![(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'M'), (-1, 1, 'S')], // both forward
        vec![(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'S'), (-1, 1, 'M')], // first forward, second backward
        vec![(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'M'), (-1, 1, 'S')], // first backward, second forward
        vec![(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'S'), (-1, 1, 'M')], // both backward
    ];

    let mut all_xmas = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col) != 'A' {
                continue;
            }

            'variant: for variant in word_variants.iter() {
                for &(row_vector, col_vector, expected_char) in variant.iter() {
                    let next_row = row as isize + row_vector;
                    let next_col = col as isize + col_vector;

                    if let Some(next_char) = grid.iget(next_row, next_col) {
                        if next_char != expected_char {
                            continue 'variant;
                        }
                    } else {
                        continue 'variant;
                    }
                }

                all_xmas += 1;
            }
        }
    }

    all_xmas
}
