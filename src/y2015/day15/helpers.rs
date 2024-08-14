use regex::Regex;

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    capacity: i16,
    durability: i16,
    flavor: i16,
    texture: i16,
    calories: i16,
}

pub fn parse_ingredient(s: &String) -> Ingredient {
    let re = Regex::new(r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let captured = re.captures(s).unwrap();

    let name = captured[1].parse::<String>().unwrap();
    let capacity = captured[2].parse::<i16>().unwrap();
    let durability = captured[3].parse::<i16>().unwrap();
    let flavor = captured[4].parse::<i16>().unwrap();
    let texture = captured[5].parse::<i16>().unwrap();
    let calories = captured[6].parse::<i16>().unwrap();

    Ingredient {
        name,
        capacity,
        durability,
        flavor,
        texture,
        calories,
    }
}

pub fn get_cookie_score(ingredients: &Vec<Ingredient>, amounts: &Vec<i16>) -> (i64, i16) {
    let len = ingredients.len();

    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;
    let mut total_calories = 0;

    for i in 0..len {
        let ingredient = &ingredients[i];
        let amount = *(&amounts[i]);

        total_capacity += ingredient.capacity * amount;
        total_durability += ingredient.durability * amount;
        total_flavor += ingredient.flavor * amount;
        total_texture += ingredient.texture * amount;
        total_calories += ingredient.calories * amount;
    }

    let final_score = total_capacity.max(0) as i64
        * total_durability.max(0) as i64
        * total_flavor.max(0) as i64
        * total_texture.max(0) as i64;

    return (final_score, total_calories);
}