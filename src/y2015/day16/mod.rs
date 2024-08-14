use crate::parse_input;

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let aunts = lines.iter().map(|line| parse_aunt(line)).collect::<Vec<Aunt>>();

    let wanted = Aunt {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let filtered_out: Vec<&Aunt> = aunts.iter().filter(|aunt| {
        if let Some(children) = aunt.children {
            if children != wanted.children.unwrap() {
                return false;
            }
        }

        if let Some(cats) = aunt.cats {
            if cats <= wanted.cats.unwrap() {
                return false;
            }
        }

        if let Some(samoyeds) = aunt.samoyeds {
            if samoyeds != wanted.samoyeds.unwrap() {
                return false;
            }
        }

        if let Some(pomeranians) = aunt.pomeranians {
            if pomeranians >= wanted.pomeranians.unwrap() {
                return false;
            }
        }

        if let Some(akitas) = aunt.akitas {
            if akitas != wanted.akitas.unwrap() {
                return false;
            }
        }

        if let Some(vizslas) = aunt.vizslas {
            if vizslas != wanted.vizslas.unwrap() {
                return false;
            }
        }

        if let Some(goldfish) = aunt.goldfish {
            if goldfish >= wanted.goldfish.unwrap() {
                return false;
            }
        }

        if let Some(trees) = aunt.trees {
            if trees <= wanted.trees.unwrap() {
                return false;
            }
        }

        if let Some(cars) = aunt.cars {
            if cars != wanted.cars.unwrap() {
                return false;
            }
        }

        if let Some(perfumes) = aunt.perfumes {
            if perfumes != wanted.perfumes.unwrap() {
                return false;
            }
        }

        return true;
    }).collect();

    dbg!(filtered_out);
}
