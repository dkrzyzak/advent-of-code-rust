pub fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        let joined = line.iter().collect::<String>();
        println!("{joined}");
    }
}

pub fn is_corner(x: usize, y: usize, size: usize) -> bool {
    (x == size - 1 && y == size - 1)
        || (x == size - 1 && y == 0)
        || (x == 0 && y == size - 1)
        || (x == 0 && y == 0)
}

pub fn count_neighbours(x: usize, y: usize, grid: &Vec<Vec<char>>, grid_size: &usize) -> (u8, u8) {
    let mut on: u8 = 0;
    let mut off: u8 = 0;

    let neigh_vectors: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (vx, vy) in neigh_vectors {
        let n_pos = (x as isize + vx, y as isize + vy);

        if n_pos.0 < 0
            || n_pos.1 < 0
            || n_pos.0 >= *grid_size as isize
            || n_pos.1 >= *grid_size as isize
        {
            off += 1;
            continue;
        }

        let n = grid[n_pos.0 as usize][n_pos.1 as usize];
        if n == '#' {
            on += 1
        } else {
            off += 1
        }
    }

    (on, off)
}
