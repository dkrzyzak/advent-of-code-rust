use lazy_static::lazy_static;
use regex::Regex;

use super::Entity;

lazy_static! {
    static ref CHANGE_DIR: Regex = Regex::new(r"\$ cd (.+)").unwrap();
    static ref LIST: Regex = Regex::new(r"\$ ls").unwrap();
    static ref DIR: Regex = Regex::new(r"dir (\w+)").unwrap();
    static ref FILE: Regex = Regex::new(r"(\d+) ([\w.]+)").unwrap();
}

pub fn traverse(instructions: &Vec<String>) -> Entity {
    let mut root = Entity::new_dir(String::from("/"));
    let mut path: Vec<String> = vec![String::from("/")];
    let mut current_dir = &mut root;

    for instr in instructions.iter() {
        if let Some(captured) = CHANGE_DIR.captures(instr) {
            let parsed_dirname = captured[1].to_string();

            match parsed_dirname.as_str() {
                "/" => {
                    path.clear();
                    path.push(String::from("/"));
                    current_dir = &mut root;
                }
                ".." => {
                    path.pop();
                    current_dir = get_dir_by_path(&mut root, &path);
                }
                _ => {
                    current_dir = current_dir.get_dir(&parsed_dirname).expect("Missing dir");
                    path.push(parsed_dirname);
                }
            }
        }

        if LIST.is_match(instr) {
            // println!("List");
        }

        if let Some(captured) = DIR.captures(instr) {
            let parsed_dirname = captured[1].to_string();
            current_dir.add_entity(parsed_dirname.clone(), Entity::new_dir(parsed_dirname));
        }

        if let Some(captured) = FILE.captures(instr) {
            let size = captured[1].parse::<usize>().unwrap();
            let name = captured[2].parse::<String>().unwrap();

            current_dir.add_entity(name.clone(), Entity::new_file(name, size));
        }
    }

    root
}

fn get_dir_by_path<'a>(root: &'a mut Entity, path: &Vec<String>) -> &'a mut Entity {
    let mut current = root;
    for segment in path.iter().skip(1) {
        current = current.get_dir(segment).expect("Missing node");
    }

    current
}
