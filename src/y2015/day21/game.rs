#[derive(Debug)]
pub struct Player {
    pub hit_points: i16,
    pub damage: i16,
    pub armor: i16,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            hit_points: 100,
            damage: 0,
            armor: 0,
        }
    }
}

pub fn reset_players(player: &mut Player, boss: &mut Player) {
    *player = Player::default();
    *boss = Player {
        hit_points: 100,
        damage: 8,
        armor: 2,
    }
}

pub fn fight(player: &mut Player, boss: &mut Player) -> bool {
    let mut turn = true;

    while player.hit_points > 0 && boss.hit_points > 0 {
        if turn {
            let damage = player.damage - boss.armor;
            boss.hit_points -= damage.max(1);
        } else {
            let damage = boss.damage - player.armor;
            player.hit_points -= damage.max(1);
        }

        turn = !turn;
    }

    return boss.hit_points <= 0;
}