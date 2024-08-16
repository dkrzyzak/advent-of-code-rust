use regex::Regex;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub cost: u16,
    pub damage: i16,
    pub armor: i16,
}

pub fn get_shop_inventory(lines: &Vec<String>) -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let mut iter = lines.iter();

    let mut weapons = Vec::<Item>::new();
    let item_reg = Regex::new(r"(\w+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();

    while let Some(line) = iter.next() {
        if line == "" {
            // ended parsing weapons
            break; 
        }

        if line.contains("Weapons") {
            // table header
            continue;
        }

        let captured = item_reg.captures(line).unwrap();
        let name = captured[1].parse::<String>().unwrap();
        let cost = captured[2].parse::<u16>().unwrap();
        let damage = captured[3].parse::<i16>().unwrap();
        let armor = captured[4].parse::<i16>().unwrap();

        weapons.push(Item { name, cost, damage, armor });
    }

    let mut armors = Vec::<Item>::new();
    while let Some(line) = iter.next() {
        if line == "" {
            // ended parsing weapons
            break; 
        }

        if line.contains("Armor") {
            // table header
            continue;
        }

        let captured = item_reg.captures(line).unwrap();
        let name = captured[1].parse::<String>().unwrap();
        let cost = captured[2].parse::<u16>().unwrap();
        let damage = captured[3].parse::<i16>().unwrap();
        let armor = captured[4].parse::<i16>().unwrap();

        armors.push(Item { name, cost, damage, armor });
    }

    let ring_regex = Regex::new(r"^(\w+\s\+\d)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let mut rings = Vec::<Item>::new();

    while let Some(line) = iter.next() {
        if line == "" {
            // ended parsing weapons
            break; 
        }

        if line.contains("Armor") {
            // table header
            continue;
        }

        let captured = ring_regex.captures(line).unwrap();
        let name = captured[1].parse::<String>().unwrap();
        let cost = captured[2].parse::<u16>().unwrap();
        let damage = captured[3].parse::<i16>().unwrap();
        let armor = captured[4].parse::<i16>().unwrap();

        rings.push(Item { name, cost, damage, armor });
    }

    (weapons, armors, rings)
    
}