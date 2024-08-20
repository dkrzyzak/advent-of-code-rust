use std::iter::repeat;

use regex::Regex;

pub fn extract_input(lines: &Vec<String>) -> Vec<(String, Vec<u8>)> {
    let mut data: Vec<(String, Vec<u8>)> = Vec::new();
    let reg = Regex::new(r"([\?\.#]+) ([,\d]+)").unwrap();

    for line in lines.iter() {
        let captured = reg.captures(line).unwrap();
        let springs = captured[1]
            .parse::<String>()
            .unwrap()
            .replace("#", "1")
            .replace(".", "0")
            .replace("?", "x");

        let springs_unfolded = repeat(springs.clone()).take(5).collect::<Vec<_>>();
        let springs_unfolded = springs_unfolded.join("x");

        let contiguous_groups = captured[2]
            .parse::<String>()
            .unwrap()
            .split(",")
            .into_iter()
            .map(|number| number.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        let unfolded_groups = repeat(contiguous_groups.clone()).take(5).flat_map(|v| v).collect::<Vec<_>>();

        data.push((springs, contiguous_groups));
        // data.push((springs_unfolded, unfolded_groups));
    }

    data
}

pub fn get_correct_regex(contiguous_groups: &Vec<u8>) -> Regex {
    // something like ^0*1{1}0+1{1}0+1{3}0*$
    let groups = contiguous_groups
        .iter()
        .map(|group| format!("1{{{}}}", group))
        .collect::<Vec<_>>()
        .join("0+");
    let regex_str = format!("^0*{}0*$", groups);

    let reg = Regex::new(&regex_str).unwrap();

    reg
}

pub fn get_acceptable_regex(contiguous_groups: &Vec<u8>) -> Regex {
    // something like ^[0x]*[1x]{1}[0x]+[1x]{1}[0x]+[1x]{3}[0x]*$
    let groups = contiguous_groups
        .iter()
        .map(|group| format!("[1x]{{{}}}", group))
        .collect::<Vec<_>>()
        .join("[0x]+");
    let regex_str = format!("^[0x]*{}[0x]*$", groups);
    let reg = Regex::new(&regex_str).unwrap();

    reg
}
