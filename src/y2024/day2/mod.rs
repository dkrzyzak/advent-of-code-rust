use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let levels: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|el| el.parse().unwrap())
                .collect()
        })
        .collect();

    let levels_safety = levels.iter().map(|level| is_safe(level) as usize).sum::<usize>();
    println!("Safety: {:?}", levels_safety);
}

fn is_safe(level: &Vec<u32>) -> bool {
    let len = level.len();
    let is_increasing = level.last().unwrap() > level.first().unwrap();
    let mut prev = level[0];

    for index in 1..len {
        let value = level[index];

        // Any two adjacent levels differ by at least one and at most three.
        let diff = value.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false;
        }

        // The levels are either all increasing or all decreasing.
        if is_increasing {
            if value <= prev {
                return false;
            }
        } else {
            if value > prev {
                return false;
            }
        }

        prev = value;
    }

    return true;
    
}