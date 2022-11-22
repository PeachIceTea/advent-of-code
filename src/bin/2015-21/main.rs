use std::{cmp, ptr};

use serde_derive::Deserialize;
use serde_scan::scan;

struct Item {
    _name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn new(name: String, cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            _name: name,
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Deserialize)]
struct Character {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn new_from_input(input: &str) -> Character {
        scan!("Hit Points: {}\nDamage: {}\nArmor: {}" <- input).unwrap()
    }

    fn new_player(damage: i32, armor: i32) -> Character {
        Character {
            hitpoints: 100,
            damage,
            armor,
        }
    }

    fn wins_fight_against(&self, opponent: &Character, opponent_starts: bool) -> bool {
        let mut self_health = self.hitpoints;
        let mut opponent_health = opponent.hitpoints;
        let mut opponents_turn = opponent_starts;
        while self_health > 0 && opponent_health > 0 {
            if opponents_turn {
                self_health -= cmp::max(1, opponent.damage - self.armor);
            } else {
                opponent_health -= cmp::max(1, self.damage - opponent.armor);
            }
            opponents_turn = !opponents_turn;
        }

        self_health > 0
    }
}

fn main() {
    let input = std::fs::read_to_string("input/2015/21.txt").expect("input should exist");

    let boss = Character::new_from_input(&input);

    let weapons = vec![
        Item::new(String::from("Dagger"), 8, 4, 0),
        Item::new(String::from("Shortsword"), 10, 5, 0),
        Item::new(String::from("Warhammer"), 25, 6, 0),
        Item::new(String::from("Longsword"), 40, 7, 0),
        Item::new(String::from("Greataxe"), 74, 8, 0),
    ];

    let armors = vec![
        Item::new(String::from("Nothing"), 0, 0, 0),
        Item::new(String::from("Leather"), 13, 0, 1),
        Item::new(String::from("Chainmail"), 31, 0, 2),
        Item::new(String::from("Splintmail"), 53, 0, 3),
        Item::new(String::from("Bandedmail"), 75, 0, 4),
        Item::new(String::from("Platemail"), 102, 0, 5),
    ];

    let rings = vec![
        Item::new(String::from("Nothing"), 0, 0, 0),
        Item::new(String::from("Nothing"), 0, 0, 0),
        Item::new(String::from("Damage +1"), 25, 1, 0),
        Item::new(String::from("Damage +2"), 50, 2, 0),
        Item::new(String::from("Damage +3"), 100, 3, 0),
        Item::new(String::from("Defense +1"), 20, 0, 1),
        Item::new(String::from("Defense +2"), 40, 0, 2),
        Item::new(String::from("Defense +3"), 80, 0, 3),
    ];

    let mut lowest_cost_to_win = -1;
    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ptr::eq(ring1, ring2) {
                        continue;
                    }

                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    let player = Character::new_player(
                        weapon.damage + ring1.damage + ring2.damage,
                        armor.armor + ring1.armor + ring2.armor,
                    );
                    if (lowest_cost_to_win == -1 || cost < lowest_cost_to_win)
                        && player.wins_fight_against(&boss, false)
                    {
                        lowest_cost_to_win = cost;
                    }
                }
            }
        }
    }

    let mut highest_cost_to_loose = -1;
    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ptr::eq(ring1, ring2) {
                        continue;
                    }

                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    let player = Character::new_player(
                        weapon.damage + ring1.damage + ring2.damage,
                        armor.armor + ring1.armor + ring2.armor,
                    );
                    if cost > highest_cost_to_loose && !player.wins_fight_against(&boss, false) {
                        highest_cost_to_loose = cost;
                    }
                }
            }
        }
    }

    println!(
        "You have to spent at least {} gold to win the fight against the boss.",
        lowest_cost_to_win
    );
    println!(
        "If the boss decides your items, you could spend {} gold and still loose.",
        highest_cost_to_loose
    );
}
