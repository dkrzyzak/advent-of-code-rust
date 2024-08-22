// -> (cycle, offset)
pub fn find_cycle(loads: &Vec<usize>) -> Option<(&[usize], usize)> {
    // dont start at 1, to avoid miscalculations when first few elements are equal
    for i in 5..loads.len() {
        let mut current_offset = 0;

        'inner: while current_offset < i {
            if loads[current_offset] != loads[i] {
                current_offset += 1;
                continue 'inner;
            }

            let mut j = 1;
            while j < i - current_offset {
                if loads[current_offset + j] != loads[i + j] {
                    current_offset += 1;
                    continue 'inner;
                } else {
                    j += 1;
                }
            }

            return Some((&loads[current_offset..i], current_offset));
        }
    }

    None
}

pub fn get_load_on_iteration(iter: usize, cycles: &[usize], cycle_offset: usize) -> usize {
    let load_index = (iter - (cycle_offset + 1)) % cycles.len();

    cycles[load_index]
}
