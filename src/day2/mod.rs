#![allow(dead_code)]

use super::read::read_input_file;
use regex::Regex;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct GameData {
    game_id: u32,
    games: Vec<Game>,
}

pub fn task1() {
    let lines = read_input_file("day2");

    let game_data = lines
        .iter()
        .map(|line| {
            let (id_string, game_string) = line.split_once(": ").unwrap_or_default();
            let game_id = parse_game_id(id_string);
            let games: Vec<Game> = game_string
                .split("; ")
                .map(|game| parse_game_colors(game))
                .collect();

            GameData { game_id, games }
        })
        .fold(0, |total, current| {
            let GameData { game_id, games } = current;
            let is_incorrect = games
                .iter()
                .any(|game| (game.red > 12 || game.green > 13 || game.blue > 14));

            if is_incorrect {
                total
            } else {
                total + game_id
            }
        });

    println!("SUM: {:?}", game_data);
}

pub fn task2() {
let lines = read_input_file("day2");

    let game_data = lines
        .iter()
        .map(|line| {
            let (id_string, game_string) = line.split_once(": ").unwrap_or_default();
            let game_id = parse_game_id(id_string);
            let games: Vec<Game> = game_string
                .split("; ")
                .map(|game| parse_game_colors(game))
                .collect();

            GameData { game_id, games }
        })
        .fold(0, |total, current| {
            let GameData { games, game_id: _ } = current;
            let mut minimal_game = Game {
               red: 0,
               blue: 0,
               green: 0,
            };

            games.iter().for_each(|game| {
               if game.red > minimal_game.red {
                  minimal_game.red = game.red;
               }

               if game.green > minimal_game.green {
                  minimal_game.green = game.green;
               }

               if game.blue > minimal_game.blue {
                  minimal_game.blue = game.blue;
               }
            });

            let minimal_power = minimal_game.red * minimal_game.green * minimal_game.blue;

            total + minimal_power
        });

    println!("SUM: {:?}", game_data);
}

fn parse_game_id(input: &str) -> u32 {
    let re = Regex::new(r"Game (\d+)").expect("Failed to create regex");

    let number = re
        .captures(input)
        .and_then(|captures| captures.get(1))
        .and_then(|number_str| number_str.as_str().parse::<u32>().ok())
        .unwrap_or_else(|| 0);

    number
}

fn parse_game_colors(input: &str) -> Game {
    let mut result = Game {
        red: 0,
        green: 0,
        blue: 0,
    };

    vec![
        (r"(\d+) red", "red"),
        (r"(\d+) green", "green"),
        (r"(\d+) blue", "blue"),
    ]
    .iter()
    .map(|(regex, color)| (Regex::new(regex).expect(""), color))
    .for_each(|(regex, &color)| {
        let value = regex
            .captures(input)
            .and_then(|captures| captures.get(1))
            .and_then(|number_str| number_str.as_str().parse::<u32>().ok())
            .unwrap_or_else(|| 0);

        if value > 0 {
            match color {
                "red" => {
                    result.red = value;
                }
                "green" => {
                    result.green = value;
                }
                "blue" => {
                    result.blue = value;
                }
                _ => {}
            }
        }
    });

    result
}
