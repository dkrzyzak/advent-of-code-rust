use crate::parse_input;

mod game;
mod shop;

use game::*;
use shop::*;

pub fn task() {
    let lines = parse_input!();
    let (weapons, armors, rings) = get_shop_inventory(&lines);

    let mut player = Player::default();
    let mut boss = Player::default();

    let mut biggest_failing_cost: u16 = 0;
    let mut smallest_cost = u16::MAX;

    for weapon in weapons.iter() {
        for a in -1..armors.len() as isize {
            for r1 in -1..rings.len() as isize {
                for r2 in -1..rings.len() as isize {
                    reset_players(&mut player, &mut boss);

                    let mut build = Vec::<String>::new();
                    
                    let mut total_cost = weapon.cost;
                    player.damage += weapon.damage;
                    build.push(weapon.name.clone());
                    

                    if a != -1 {
                        let armor = &armors[a as usize];
                        player.armor += armor.armor;
                        total_cost += armor.cost;
                        build.push(armor.name.clone());
                    }

                    if r1 != -1 {
                        let ring1 = &rings[r1 as usize];
                        player.armor += ring1.armor;
                        player.damage += ring1.damage;
                        total_cost += ring1.cost;
                        build.push(ring1.name.clone());
                    }

                    if r2 != -1 && r2 != r1 {
                        let ring2 = &rings[r2 as usize];
                        player.armor += ring2.armor;
                        player.damage += ring2.damage;
                        total_cost += ring2.cost;
                        build.push(ring2.name.clone());
                    }

                    let won = fight(&mut player, &mut boss);
                    
                    if won && total_cost < smallest_cost {
                        smallest_cost = total_cost;
                    }

                    if !won && total_cost > biggest_failing_cost {
                        biggest_failing_cost = total_cost;
                    }

                    player = Player::default();
                }
            }
        }
    }

    println!("Cheapest winning build: {}", smallest_cost);
    println!("Most expensive losing build: {}", biggest_failing_cost);
}

// let mut player = Player {
//         hit_points: 8,
//     damage: 5,
//     armor: 5,
// };

// let mut boss = Player {
//     hit_points: 12,
//     damage: 7,
//     armor: 2,
// };
