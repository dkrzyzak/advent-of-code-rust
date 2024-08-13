use regex::Regex;

use crate::parse_input;

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed_kms: u32,
    fly_time: u32,
    fly_period: u32,
    rest_time: u32,
    rest_period: u32,
    is_resting: bool,

    total_distance: u32,
    points: u32,
}

pub fn task() {
    let lines = parse_input!();

    let mut reindeers: Vec<Reindeer> = lines.iter().map(|line| {
        let reg = Regex::new(r"^(\w+) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds\.").unwrap();
        let captured = reg.captures(line).unwrap();

        let name = captured[1].parse::<String>().unwrap();
        let speed = captured[2].parse::<u32>().unwrap();
        let fly_time = captured[3].parse::<u32>().unwrap();
        let rest_time = captured[4].parse::<u32>().unwrap();

        Reindeer {
            name,
            speed_kms: speed,
            fly_time,
            fly_period: 0,
            rest_time,
            rest_period: 0,
            is_resting: false,
            points: 0,
            total_distance: 0,
        }
    }).collect();


    let cutoff: u32 = 2503;

    // let first_phase_winner = reindeers.iter_mut().map(|reindeer| get_reindeer_distance_after(&cutoff, reindeer)).max().unwrap();

    race(&mut reindeers, &cutoff);
    let second_phase_winner = reindeers.iter().map(|reindeer| reindeer.points).max().unwrap();
    println!("Second phase winner: {second_phase_winner}");

}

fn get_reindeer_distance_after(seconds: &u32, reindeer: &mut Reindeer) -> u32 {
    let mut elapsed: u32 = 0;

    while elapsed < *seconds {
        reindeer_tick(reindeer);
        elapsed += 1;
    }

    reindeer.total_distance
}

fn race(reindeers: &mut Vec<Reindeer>, cutoff: &u32) {
    let mut elapsed = 0;

    while elapsed < *cutoff {
        for reindeer in reindeers.iter_mut() {
            reindeer_tick(reindeer);
        }

        elapsed += 1;

        let furthest_distance = reindeers.iter().map(|reindeer| reindeer.total_distance).max().unwrap();

        for reindeer in reindeers.iter_mut() {
            if reindeer.total_distance == furthest_distance {
                reindeer.points += 1;
            }
        }
    }
}

fn reindeer_tick(reindeer: &mut Reindeer) {
    if reindeer.is_resting {
        reindeer.rest_period += 1;
        if reindeer.rest_period == reindeer.rest_time {
            reindeer.is_resting = false;
            reindeer.rest_period = 0;
        }
    } else {
        reindeer.fly_period += 1;
        reindeer.total_distance += reindeer.speed_kms;

        if reindeer.fly_period == reindeer.fly_time {
            reindeer.is_resting = true;
            reindeer.fly_period = 0;
        }
    }
}
