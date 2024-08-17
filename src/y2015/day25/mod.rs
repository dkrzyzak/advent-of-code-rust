use std::fmt::Display;

pub fn task() {
    let mut infinite_grid: Vec<Vec<u64>> = Vec::new();
    let mut first_row: Vec<u64> = Vec::new();
    let mut value = 20151125;
    first_row.push(value);
    infinite_grid.push(first_row);

    let mut x = 0;
    let mut y = 1;

    let desired_row = 2981;
    let desired_col = 3075;

    loop {
        value = get_next_value(value);

        if x == 0 {
            let mut row = Vec::new();
            row.push(value);
            infinite_grid.push(row);
            y -= 1;
            x += 1;
        } else if y == 0 {
            infinite_grid[y].push(value);
            y = x + 1;
            x = 0;
        } else {
            infinite_grid[y].push(value);
            y -= 1;
            x += 1;
        }

        if x == desired_row && y == desired_col {
            break;
        }


    }

    println!("Value at row {} col {}: {}", desired_row, desired_col, infinite_grid[desired_row - 1][desired_col - 1])

}

fn get_next_value(current_value: u64) -> u64 {
    (current_value * 252533) % 33554393
}

fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for col in row {
            print!("{:>10} ", col);
        }
        print!("\n");
    }
}