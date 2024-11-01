use crate::parse_input;

pub mod extract;
use extract::*;

pub fn task() {
    let lines = parse_input!();

    let (mut stacks, moves) = extract_input(&lines);

    for (amount, from, to) in moves.iter() {
        // PART 1:
        // for _ in 0..*amount {
        //     let current_crate = stacks[*from as usize].pop().unwrap();
        //     stacks[*to as usize].push(current_crate);
        // }

        if let Some(from_vector) = stacks.get_mut(*from as usize) {
            let items_to_move = from_vector.split_off(from_vector.len() - *amount as usize);
            if let Some(to_vector) = stacks.get_mut(*to as usize) {
                to_vector.extend(items_to_move);
            }
        }
    }

    let message = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Final message: {}", message);
}
