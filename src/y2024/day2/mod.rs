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

    let levels_safety_1 = levels.iter().map(|level| get_unsafe_index(level).is_none() as u32).sum::<u32>();
    println!("Safe (Pt 1): {:?}", levels_safety_1);

    let levels_safety_2 = levels
        .iter()
        .map(|level| {
            if let Some(problematic_index) = get_unsafe_index(level) {
                // we will try to remove either of the problematic items
                let mut new_level_a = level.clone();
                let mut new_level_b = level.clone();

                new_level_a.remove(problematic_index);
                new_level_b.remove(problematic_index - 1);

                let result = get_unsafe_index(&new_level_a).is_none()
                    || get_unsafe_index(&new_level_b).is_none();

                return if result { 1 } else { 0 };
            } else {
                return 1;
            }
        })
        .sum::<u32>();
    println!("Safe (Pt 2): {:?}", levels_safety_2);
}

// if level is safe -> returns None
// if level is unsafe -> returns index of first unsafe index
fn get_unsafe_index(level: &Vec<u32>) -> Option<usize> {
    let len = level.len();
    let is_increasing = level.last().unwrap() > level.first().unwrap();
    let mut prev = level[0];

    for index in 1..len {
        let value = level[index];

        // Any two adjacent levels differ by at least one and at most three.
        let diff = value.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return Some(index);
        }

        // The levels are either all increasing or all decreasing.
        if is_increasing {
            if value <= prev {
                return Some(index);
            }
        } else {
            if value >= prev {
                return Some(index);
            }
        }

        prev = value;
    }

    return None;
}
