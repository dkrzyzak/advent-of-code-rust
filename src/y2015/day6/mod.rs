use regex::Regex;

use crate::read::read_input_file;

fn extract_from_line(line: &String) -> (String, (usize, usize), (usize, usize)) {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    if let Some(captured) = re.captures(line) {
        let mode = captured[1].parse::<String>().unwrap();
        let x1 = captured[2].parse::<usize>().unwrap();
        let y1 = captured[3].parse::<usize>().unwrap();
        let x2 = captured[4].parse::<usize>().unwrap();
        let y2 = captured[5].parse::<usize>().unwrap();

        return (mode, (x1, y1), (x2, y2));
    }

    return (String::new(), (0, 0), (0, 0));
}

pub fn task() {
    let lines = read_input_file("y2015", "day6");

    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for line in lines.iter() {
        let (mode, p1, p2) = extract_from_line(line);

        for x in p1.0..=p2.0 {
            for y in p1.1..=p2.1 {
                match mode.as_str() {
                    "turn on" => {
                        grid[x][y] += 1;
                    }
                    "turn off" => {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                    "toggle" => {
                        grid[x][y] += 2;
                    }
                    _ => {}
                }
            }
        }
    }

    let sum: usize = grid.iter().map(|row| row.iter().sum::<usize>()).sum();

    println!("sum: {sum}");
}
