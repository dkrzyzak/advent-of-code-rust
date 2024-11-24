use std::collections::HashSet;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let stream = lines.first().unwrap();
    let stream_chars = stream.chars().collect::<Vec<_>>();

    let start_of_packet = get_start_index(&stream_chars, 4);
    let start_of_message = get_start_index(&stream_chars, 14);
    println!("start_of_packet = {}, start_of_message = {}", start_of_packet, start_of_message);
}

pub fn get_start_index(stream: &Vec<char>, marker_len: usize) -> usize {
    let stream_len = stream.len();
    let mut i = 0usize;

    loop {
        let four_slice = &stream[i..i+marker_len];
        let consequent_chars: HashSet<&char> = HashSet::from_iter(four_slice.iter());
        if consequent_chars.len() != marker_len {
            i += 1;
        } else {
            return i + marker_len;
        }
    }
}
