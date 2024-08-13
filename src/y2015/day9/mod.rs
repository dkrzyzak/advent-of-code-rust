use std::{collections::HashMap, u32};
use itertools::Itertools;
use regex::Regex;


use crate::parse_input;

pub fn task() {
    let lines = parse_input!();

    // { Dublin: [ (London, 464), (Belfast, 141) ] }
    let mut distances: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    let reg = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    for line in lines {
        let captured = reg.captures(&line).unwrap();
        let city1 = captured[1].parse::<String>().unwrap();
        let city2 = captured[2].parse::<String>().unwrap();
        let distance = captured[3].parse::<u32>().unwrap();

        distances
            .entry(city1.clone())
            .or_insert_with(Vec::new)
            .push((city2.clone(), distance));
        distances
            .entry(city2.clone())
            .or_insert_with(Vec::new)
            .push((city1.clone(), distance));
    }

    let cities: Vec<&String> = distances.keys().collect();
    let perms = cities.iter().permutations(cities.len()); // TODO: implement yourself
    let distances: Vec<u32> = perms.map(|perm| {
        let mut total_distance: u32 = 0;
        for i in 0..perm.len() - 1 {
            let current_city = *perm[i];
            let next_city = *perm[i + 1];

            let distances_map = distances.get(current_city).unwrap();
            let distance = distances_map.iter().find(|(c, _)| c == next_city).unwrap().1;
            total_distance += distance;
        }

        return total_distance;
    }).collect();

    let smallest = distances.iter().min().unwrap();
    let biggest = distances.iter().max().unwrap();

    println!("smallest: {smallest}, biggest: {biggest}");
}
