pub fn extract_mirrors(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    let mut mirrors: Vec<Vec<Vec<char>>> = Vec::new();
    let mut iter = lines.iter();
    let mut mirror_index = 0;

    mirrors.push(Vec::new());

    while let Some(line) = iter.next() {
        if line == "" {
            mirror_index += 1;
            mirrors.push(Vec::new());
            continue;
        }

        let row = line.chars().collect::<Vec<_>>();
        mirrors[mirror_index].push(row);
    }

    mirrors
}

pub fn print_mirror(mirror: &Vec<Vec<char>>) {
    for row in mirror.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}

pub fn horizontal_index(mirror: &Vec<Vec<char>>) -> Option<usize> {
    for row in 0..mirror.len() - 1 {
        let current_row = &mirror[row];
        let next_row = &mirror[row + 1];

        if current_row != next_row {
            continue;
        }
        let mut search_back = row as isize - 1;
        let mut search_forward = row + 2;
        let mut search_success = true;

        'mirror_verification: while search_back >= 0 && search_forward < mirror.len() {
            if &mirror[search_back as usize] != &mirror[search_forward] {
                search_success = false;
                break 'mirror_verification;
            }
            search_back -= 1;
            search_forward += 1;
        }

        if search_success {
            return Some(row + 1); // enumerating from 1;
        }
    }

    None
}

pub fn vertical_index(mirror: &Vec<Vec<char>>) -> Option<usize> {
    let cols_amount = mirror[0].len();

    'outer: for col in 0..cols_amount - 1 {
        let mut was_col_mirrored = true;
        // let mut smudges_left = 1;

        for row in mirror {
            if row[col] != row[col + 1] {
                // if smudges_left == 0 {
                    continue 'outer;
                // }
                // smudges_left -= 1;
            }
        }

        let mut search_back = col as isize - 1;
        let mut search_forward = col + 2;
        let mut search_success = true;

        'mirror_verification: while search_back >= 0 && search_forward < cols_amount {
            for row in mirror {
                if row[search_back as usize] != row[search_forward] {
                    search_success = false;
                    break 'mirror_verification;
                }
            }

            search_back -= 1;
            search_forward += 1;
        }

        if search_success {
            return Some(col + 1); // enumerating from 1
        }
    }

    None
}
