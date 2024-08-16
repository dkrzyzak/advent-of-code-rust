pub const SPELLS: &'static [&str; 5] = &["missile", "drain", "shield", "poison", "recharge"];


#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub hit_points: i32,
    pub mana: i32,
    pub mana_spent: i32,

    pub shield: u8,
    pub poison: u8,
    pub recharge: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Boss {
    pub hit_points: i32,
    pub damage: i32,
}

// recursive fn
pub fn next_round(
    spell: &str,
    _player: &mut Player,
    _boss: &mut Boss,
    _spells_sequence: &Vec<&str>,
    least_mana: &mut i32,
) {
    let mut spells_sequence = _spells_sequence.clone();
    spells_sequence.push(spell);

    let mut player = _player.clone();
    let mut boss = _boss.clone();

    // PART 2 rule
    player.hit_points -= 1;

    if player.hit_points < 1 {
        // lost because of the PART 2 rule
        return;
    }

    queue_spell(spell, &mut player, &mut boss);

    if player.mana < 0 {
        return;
    }

    // trick to short-circuit ineffective routes
    if player.mana_spent > *least_mana {
        return;
    }

    // BOSS ROUND
    // 1. apply active effects
    let has_shield = apply_active_spells(&mut player, &mut boss);

    if boss.hit_points <= 0 {
        // WON!
        if player.mana_spent < *least_mana {
            println!("Current least mana spent: {}", player.mana_spent);
            println!("Won with a sequence: {:?}", spells_sequence);
            *least_mana = player.mana_spent;
        }

        return;
    }

    // 2. attack player
    // no need to do .max(1), because boss.damage == 9 so it would still be > 0 after subtraction
    let damage = boss.damage - (if has_shield { 7 } else { 0 });
    player.hit_points -= damage;

    // PLAYER TURN

    if player.hit_points <= 0 {
        // LOST!
        return;
    }

    apply_active_spells(&mut player, &mut boss);

    // recursively call all next possible spells in this route
    for spell in SPELLS {
        if *spell == "shield" && player.shield > 0 {
            continue;
        }

        if *spell == "poison" && player.poison > 0 {
            continue;
        }

        if *spell == "recharge" && player.recharge > 0 {
            continue;
        }

        next_round(spell, &mut player, &mut boss, &spells_sequence, least_mana);
    }
}

fn apply_active_spells(player: &mut Player, boss: &mut Boss) -> bool {
    if player.poison > 0 {
        boss.hit_points -= 3;
        player.poison -= 1;
    }

    if player.recharge > 0 {
        player.mana += 101;
        player.recharge -= 1;
    }

    let has_shield = player.shield > 0;
    if has_shield {
        player.shield -= 1;
    }

    return has_shield;
}

fn queue_spell(spell: &str, player: &mut Player, boss: &mut Boss) {
    match spell {
        "missile" => {
            player.mana -= 53;
            player.mana_spent += 53;
            boss.hit_points -= 4;
        }
        "drain" => {
            player.mana -= 73;
            player.mana_spent += 73;
            player.hit_points += 2;
            boss.hit_points -= 2;
        }
        "shield" => {
            player.mana -= 113;
            player.mana_spent += 113;
            player.shield = 6;
        }
        "poison" => {
            player.mana -= 173;
            player.mana_spent += 173;
            player.poison = 6;
        }
        "recharge" => {
            player.mana -= 229;
            player.mana_spent += 229;
            player.recharge = 5;
        }
        _ => {}
    }
}
