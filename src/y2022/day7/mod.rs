use std::collections::HashMap;

use crate::parse_input;

mod filetree;
mod traverse;

use filetree::*;
use traverse::*;

type Cache = HashMap<Vec<String>, usize>;

const TOTAL_DISK_SPACE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

pub fn task() {
    let lines = parse_input!();

    let filetree = traverse(&lines);

    let mut cache: Cache = HashMap::new();
    size_with_cache(&filetree, &mut cache, Vec::new());

    // PART 1
    // let total = cache.values().filter(|val| **val <= 100000).sum::<usize>();
    // println!("Total: {}", total);

    // PART 2
    let root_key = vec![String::from("/")];
    let root_size = cache.get(&root_key).unwrap();
    let free_space = TOTAL_DISK_SPACE - root_size;
    let missing_space = UPDATE_SIZE - free_space;

    let mut suitable_dirs = cache
        .values()
        .filter(|val| **val >= missing_space)
        .map(|val| *val)
        .collect::<Vec<_>>();
    suitable_dirs.sort();
    let smallest_suitable_dir = suitable_dirs.first().unwrap();
    println!("Smallest: {}", smallest_suitable_dir);
}

pub fn size_with_cache(entity: &Entity, cache: &mut Cache, path: Vec<String>) -> usize {
    match entity {
        Entity::File(_, size) => *size,
        Entity::Dir(name, entities) => {
            let mut current_path = path.clone();
            current_path.push(name.clone());

            if let Some(dir_size) = cache.get(&current_path) {
                return *dir_size;
            }

            let dir_size = entities
                .values()
                .map(|e| size_with_cache(e, cache, current_path.clone()))
                .sum::<usize>();
            cache.insert(current_path, dir_size);

            return dir_size;
        }
    }
}
