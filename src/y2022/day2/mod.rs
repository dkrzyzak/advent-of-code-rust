mod shapes;
use crate::parse_input;
use shapes::*;

pub fn task() {
    let lines = parse_input!();
    let rounds = extract_shapes(&lines);

    let mut total_score = 0u32;

    for (elf_shape, my_shape) in rounds.iter() {
        let result = my_shape.fight(elf_shape).value();
        let shape_value = my_shape.value();

        total_score += result as u32 + shape_value as u32;
    }

    println!("Total score: {}", total_score);
}
