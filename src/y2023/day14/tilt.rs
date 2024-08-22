pub fn tilt_north(mirror: &mut Vec<Vec<char>>) {
    let cols = mirror[0].len();
    let rows = mirror.len();

    for col in 0..cols {
        let mut last_free_space = 0;

        for row in 0..rows {
            let item = &mirror[row][col];

            match *item {
                '#' => {
                    last_free_space = row + 1;
                }
                'O' => {
                    let mut position = row;

                    while position > last_free_space {
                        (mirror[position][col], mirror[position - 1][col]) =
                            (mirror[position - 1][col], mirror[position][col]);
                        position -= 1;
                    }
                    last_free_space += 1;
                }
                _ => {}
            }
        }
    }
}

pub fn tilt_south(mirror: &mut Vec<Vec<char>>) {
    let cols = mirror[0].len();
    let rows = mirror.len();

    for col in 0..cols {
        let mut last_free_space = rows - 1;

        for row in (0..rows).rev() {
            let item = &mirror[row][col];

            match *item {
                '#' => {
                    if row > 0 {
                        last_free_space = row - 1;
                    }
                }
                'O' => {
                    let mut position = row;

                    while position < last_free_space {
                        (mirror[position][col], mirror[position + 1][col]) =
                            (mirror[position + 1][col], mirror[position][col]);
                        position += 1;
                    }
                    if last_free_space > 0 {
                        last_free_space -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn tilt_east(mirror: &mut Vec<Vec<char>>) {
    let cols = mirror[0].len();
    let rows = mirror.len();

    for row in 0..rows {
        let mut last_free_space = cols - 1;

        for col in (0..cols).rev() {
            let item = &mirror[row][col];

            match *item {
                '#' => {
                    if col > 0 {
                        last_free_space = col - 1;
                    }
                }
                'O' => {
                    let mut position = col;

                    while position < last_free_space {
                        (mirror[row][position], mirror[row][position + 1]) =
                            (mirror[row][position + 1], mirror[row][position]);
                        position += 1;
                    }

                    if last_free_space > 0 {
                        last_free_space -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn tilt_west(mirror: &mut Vec<Vec<char>>) {
    let cols = mirror[0].len();
    let rows = mirror.len();

    for row in 0..rows {
        let mut last_free_space = 0;

        for col in 0..cols {
            let item = &mirror[row][col];

            match *item {
                '#' => {
                    last_free_space = col + 1;
                }
                'O' => {
                    let mut position = col;

                    while position > last_free_space {
                        (mirror[row][position], mirror[row][position - 1]) =
                            (mirror[row][position - 1], mirror[row][position]);
                        position -= 1;
                    }
                    last_free_space += 1;
                }
                _ => {}
            }
        }
    }
}
