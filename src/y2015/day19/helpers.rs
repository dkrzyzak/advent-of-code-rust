use itertools::Itertools;
use regex::Regex;

pub fn parse_productions(lines: &Vec<String>) -> (Vec<(String, String)>, String) {
    let mut productions: Vec<(String, String)> = Vec::new();
    let mut iter = lines.iter().peekable();
    let reg = Regex::new(r"(\w+) => (\w+)").unwrap();

    while let Some(line) = iter.next() {
        if line == "" {
            break;
        }

        let captured = reg.captures(line).unwrap();
        let key = captured[1].parse::<String>().unwrap();
        let res = captured[2].parse::<String>().unwrap();

        productions.push((key, res));
    }

    let starting = iter.next().unwrap().clone();

    (productions, starting)
}

pub fn reverse_productions(productions: &Vec<(String, String)>) -> Vec<(String, String)> {
    productions
        .iter()
        .map(|(target, replacement)| (replacement.clone(), target.clone()))
        .sorted_by(|(replacement_a, target_a), (replacement_b, target_b)| {
            let diff_a = replacement_a.len() - target_a.len();
            let diff_b = replacement_b.len() - target_b.len();

            diff_b.cmp(&diff_a)
        })
        .collect::<Vec<_>>()
}
