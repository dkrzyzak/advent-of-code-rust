mod game;

use game::*;

pub fn task() {
    let mut player = Player {
        hit_points: 50,
        mana: 500,
        mana_spent: 0,

        shield: 0,
        poison: 0,
        recharge: 0,
    };

    let mut boss = Boss {
        hit_points: 51,
        damage: 9,
    };

    let mut least_mana = i32::MAX;
    let initial_sequence = Vec::new();

    // recursively explore all* possible sequences of spells (brute-force)
    // *but short-circuit ineffective ones before they get out of hand
    for spell in SPELLS {
        next_round(
            spell,
            &mut player,
            &mut boss,
            &initial_sequence,
            &mut least_mana,
        );
    }

    println!("Least mana won: {least_mana}");
}
