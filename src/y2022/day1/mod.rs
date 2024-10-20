use crate::parse_input;

pub fn task() {
    let lines = parse_input!();

    let parsed_groups = lines.iter().fold(vec![vec![]], |mut acc, item| {
        if item == "" {
            acc.push(vec![]);
        } else {
            if let Ok(value) = item.parse::<u32>() {
                acc.last_mut().unwrap().push(value);
            }
        }

        acc
    });

    let mut summed_up = parsed_groups
        .iter()
        .map(|all_calories| all_calories.iter().sum())
        .collect::<Vec<u32>>();
    summed_up.sort();
    summed_up.reverse();

    let sum_top_3 = summed_up.iter().take(3).sum::<u32>();

    println!("Top: {}", sum_top_3);
}
