use helpers::reverse_productions;
use regex::Regex;
use std::collections::HashSet;

mod helpers;

use crate::parse_input;

pub fn task() {
    let lines = parse_input!();
    let (productions, starting) = helpers::parse_productions(&lines);

    // task1(&productions, &starting);

    let prods_rev = reverse_productions(&productions);
    let molecule = starting;
    let steps = 0;

    while molecule.contains("Ar") {
        let ar_index = molecule.find("Ar").unwrap();
        let rn_index = molecule.rfind("Rn").unwrap();
        println!("{ar_index}, {rn_index}");
    }
}

fn task1(productions: &Vec<(String, String)>, starting: &String) {
    let mut results: HashSet<String> = HashSet::new();

    for (target, replacement) in productions {
        let pattern = format!("({})", target);
        let re = Regex::new(&pattern).unwrap();

        let captured_iter = re.captures_iter(&starting);
        for cap in captured_iter {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();

            let mut production = String::new();
            production.push_str(&starting[0..start]);
            production.push_str(&replacement);
            production.push_str(&starting[end..]);

            results.insert(production);
        }
    }

    println!("All productions: {}", results.len());
}

// deep wip
fn reduce(s: &String, productions: &Vec<(String, String)>) {
    for (replacement, target) in productions.iter() {
        if s.contains(replacement) {
            // if target == "e" && s != replacement {
            //     continue;
            // }

            let reduced = s.replacen(replacement, target, 1);
            // println!("[{s}] {replacement} => {target} gave {reduced}");
            reduce(&reduced, productions);
        }
    }
}
