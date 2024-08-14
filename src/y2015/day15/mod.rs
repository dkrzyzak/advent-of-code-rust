use crate::parse_input;

mod helpers;
use helpers::*;

pub fn task() {
    let lines = parse_input!();
    let ingredients = lines
        .iter()
        .map(|line| parse_ingredient(line))
        .collect::<Vec<Ingredient>>();


    let mut high_score = 0;

    for a in 0..100 {
        let mut remain = 100 - a;

        for b in 0..remain {
            remain = 100 - a - b;

            for c in 0..remain {
                remain = 100 - a - b - c;
                let d = remain;

                let amounts = vec![a, b, c, d];

                let (score, calories) = get_cookie_score(&ingredients, &amounts);

                if calories != 500 {
                    continue;
                }

                if score > high_score {
                    high_score = score;
                }
            }
        }
    }

    println!("High score: {high_score}");
}
