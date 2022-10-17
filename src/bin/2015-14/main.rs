use std::cmp::Ordering;
use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_scan::scan;

#[derive(Deserialize)]
struct Reindeer {
    speed: i32,
    endurance: i32,
    rest: i32,
}

struct ReindeerStatus<'a> {
    info: &'a Reindeer,
    traveled: i32,
    endurance: i32,
    rest: i32,
    points: i32,
}

fn main() {
    let input = std::fs::read_to_string("input/2015/14.txt").expect("input should exist");

    let mut race_reindeer = HashMap::new();
    for line in input.lines() {
        let (name, reindeer): (String, Reindeer) =
            scan!("{} can fly {} km/s for {} seconds, but then must rest for {} seconds." <- line)
                .unwrap();
        race_reindeer.insert(name, reindeer);
    }

    let results = simulate(&race_reindeer, 2503);
    let (mut name, mut distance) = (String::from(" "), 0);

    for (r_name, (r_distance, _)) in &results {
        if r_distance > &distance {
            (name, distance) = (r_name.clone(), *r_distance);
        }
    }
    println!(
        "By greatest distance {} won with a distance of {}.",
        name, distance
    );

    let mut points = 0;
    for (r_name, (_, r_points)) in &results {
        if r_points > &points {
            (name, points) = (r_name.clone(), *r_points);
        }
    }
    println!(
        "By most points {} won with a total {} points.",
        name, points
    );
}

fn simulate(reindeer: &HashMap<String, Reindeer>, length: i32) -> HashMap<String, (i32, i32)> {
    let mut simulation = HashMap::new();
    for (reindeer_name, reindeer_info) in reindeer {
        simulation.insert(
            reindeer_name,
            ReindeerStatus {
                info: reindeer_info,
                traveled: 0,
                endurance: reindeer_info.endurance,
                rest: 0,
                points: 0,
            },
        );
    }

    for _ in 1..=length {
        for (_, status) in &mut simulation {
            if status.endurance > 0 {
                status.traveled += status.info.speed;
                status.endurance -= 1;
                if status.endurance == 0 {
                    status.rest = status.info.rest;
                }
            } else {
                status.rest -= 1;
                if status.rest == 0 {
                    status.endurance = status.info.endurance;
                }
            }
        }

        let mut furthest_reindeer = Vec::new();
        for (_, status) in &mut simulation {
            if furthest_reindeer.len() == 0 {
                furthest_reindeer.push(status);
            } else {
                let comp_reindeer = furthest_reindeer.first().unwrap();
                match comp_reindeer.traveled.cmp(&status.traveled) {
                    Ordering::Equal => furthest_reindeer.push(status),
                    Ordering::Less => furthest_reindeer = vec![status],
                    Ordering::Greater => (),
                }
            }
        }
        for status in furthest_reindeer {
            status.points += 1;
        }
    }

    let mut results = HashMap::new();
    for (name, status) in simulation {
        results.insert(name.clone(), (status.traveled, status.points));
    }
    results
}
