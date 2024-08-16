#[derive(Debug)]
pub struct Spell<'a> {
    pub name: &'a str,
    pub cost: i32,
    pub damage: i32,
    pub heal_health: i32,
    pub heal_mana: i32,
    pub increase_armor: i32,
    pub remaining_rounds: u8,
}

impl<'a> Spell<'a> {
    pub const fn new() -> Self {
        Spell {
            name: "",
            cost: 0,
            damage: 0,
            heal_health: 0,
            heal_mana: 0,
            increase_armor: 0,
            remaining_rounds: 0,
        }
    }
}

pub const MAGIC_MISSILE: Spell<'static> = Spell {
    name: "Magic Missile",
    cost: 53,
    damage: 4,
    remaining_rounds: 0,
    ..Spell::new()
};

pub const DRAIN: Spell<'static> = Spell {
    name: "Drain",
    cost: 73,
    damage: 2,
    heal_health: 2,
    remaining_rounds: 0,
    ..Spell::new()
};

pub const SHIELD: Spell<'static> = Spell {
    name: "Shield",
    cost: 113,
    remaining_rounds: 5,
    increase_armor: 7,
    ..Spell::new()
};

pub const POISON: Spell<'static> = Spell {
    name: "Poison",
    remaining_rounds: 5,
    damage: 3,
    ..Spell::new()
};