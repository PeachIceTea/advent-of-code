use std::cmp;

use serde_scan::scan;

const PLAYER_STARTING_HITPOINTS: i32 = 50;
const PLAYER_STARTING_MANA: i32 = 500;

const MAGIC_MISSILE_COST: i32 = 53;
const MAGIC_MISSILE_DAMAGE: i32 = 4;

const DRAIN_COST: i32 = 73;
const DRAIN_DAMAGE: i32 = 2;
const DRAIN_AMOUNT: i32 = 2;

const SHIELD_COST: i32 = 113;
const SHIELD_DURATION: i32 = 6;
const SHIELD_AMOUNT: i32 = 7;

const POISON_COST: i32 = 173;
const POISON_DURATION: i32 = 6;
const POISON_AMOUNT: i32 = 3;

const RECHARGE_COST: i32 = 229;
const RECHARGE_DURATION: i32 = 5;
const RECHARGE_AMOUNT: i32 = 101;

#[derive(PartialEq, Clone, Debug)]
enum Class {
    Fighter,
    Mage,
}

#[derive(Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Self::MagicMissile => MAGIC_MISSILE_COST,
            Self::Drain => DRAIN_COST,
            Self::Shield => SHIELD_COST,
            Self::Poison => POISON_COST,
            Self::Recharge => RECHARGE_COST,
        }
    }
}

#[derive(Clone, Debug)]
struct Character {
    hitpoints: i32,
    damage: i32,
    armor: i32,
    mana: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    kind: Class,
}

impl Character {
    fn new_boss(input: &str) -> Character {
        let (hitpoints, damage) = scan!("Hit Points: {}\nDamage: {}" <- input).unwrap();
        Character {
            hitpoints,
            damage,
            armor: 0,
            mana: 0,
            shield: 0,
            poison: 0,
            recharge: 0,
            kind: Class::Fighter,
        }
    }

    fn new_player() -> Character {
        Character {
            hitpoints: PLAYER_STARTING_HITPOINTS,
            damage: 0,
            armor: 0,
            mana: PLAYER_STARTING_MANA,
            shield: 0,
            poison: 0,
            recharge: 0,
            kind: Class::Mage,
        }
    }

    fn is_alive(&self) -> bool {
        self.hitpoints > 0
    }

    fn handle_effects(&mut self) -> i32 {
        let mut shield = 0;

        if self.poison > 0 {
            self.hitpoints -= POISON_AMOUNT;
            self.poison -= 1;
        }

        if self.shield > 0 {
            shield = SHIELD_AMOUNT;
            self.shield -= 1;
        }

        if self.recharge > 0 {
            self.mana += RECHARGE_AMOUNT;
            self.recharge -= 1;
        }

        shield
    }

    fn attack(&mut self, target: &mut Character) {
        debug_assert!(self.kind == Class::Fighter, "Mages cannot attack!");

        self.handle_effects();
        let target_shield = target.handle_effects();

        if self.hitpoints <= 0 || target.hitpoints <= 0 {
            return;
        }

        target.hitpoints -= cmp::max(0, self.damage - target_shield + target.armor);
    }

    fn cast(&mut self, target: &mut Character, spell: Spell, hard: bool) {
        debug_assert!(self.kind == Class::Mage, "Fighters cannot cast spells!");

        if hard {
            self.hitpoints -= 1;
        }

        self.handle_effects();
        target.handle_effects();

        if !self.is_alive() || !target.is_alive() {
            return;
        }

        match spell {
            Spell::MagicMissile => {
                debug_assert!(
                    self.mana >= MAGIC_MISSILE_COST,
                    "Not enough mana to cast Magic Missile!"
                );

                self.mana -= MAGIC_MISSILE_COST;
                target.hitpoints -= MAGIC_MISSILE_DAMAGE;
            }
            Spell::Drain => {
                debug_assert!(self.mana >= DRAIN_COST, "Not enough mana to cast Drain!");

                self.mana -= DRAIN_COST;
                self.hitpoints += DRAIN_AMOUNT;
                target.hitpoints -= DRAIN_DAMAGE;
            }
            Spell::Shield => {
                debug_assert!(self.mana >= SHIELD_COST, "Not enough mana to cast Shield!");
                debug_assert!(
                    self.shield == 0,
                    "Cannot cast Shield because it is already active!"
                );

                self.mana -= SHIELD_COST;
                self.shield = SHIELD_DURATION;
            }
            Spell::Poison => {
                debug_assert!(self.mana >= POISON_COST, "Not enough mana to cast Poison!");
                debug_assert!(
                    target.poison == 0,
                    "Cannot cast Poison because it is already active!"
                );

                self.mana -= POISON_COST;
                target.poison = POISON_DURATION;
            }
            Spell::Recharge => {
                debug_assert!(
                    self.mana >= RECHARGE_COST,
                    "Not enough mana to cast Recharge!"
                );
                debug_assert!(
                    self.recharge == 0,
                    "Cannot cast Recharge because it is already active!"
                );

                self.mana -= RECHARGE_COST;
                self.recharge = RECHARGE_DURATION;
            }
        }
    }

    fn possible_spells(&self, target: &Character) -> Vec<Spell> {
        let mana = self.mana
            + if self.recharge > 0 {
                RECHARGE_AMOUNT
            } else {
                0
            };
        let mut possible_spells = Vec::new();

        if mana >= MAGIC_MISSILE_COST {
            possible_spells.push(Spell::MagicMissile);
        }

        if mana >= DRAIN_COST {
            possible_spells.push(Spell::Drain);
        }

        if mana >= SHIELD_COST && self.shield <= 1 {
            possible_spells.push(Spell::Shield);
        }

        if mana >= POISON_COST && target.poison <= 1 {
            possible_spells.push(Spell::Poison);
        }

        if mana >= RECHARGE_COST && self.recharge <= 1 {
            possible_spells.push(Spell::Recharge);
        }

        possible_spells
    }
}

enum RoundOutcome {
    StillFighting,
    PlayerWon,
    BossWon,
    Tie,
}

impl RoundOutcome {
    fn determine(player: &Character, boss: &Character) -> RoundOutcome {
        match (player.is_alive(), boss.is_alive()) {
            (true, true) => Self::StillFighting,
            (true, false) => Self::PlayerWon,
            (false, true) => Self::BossWon,
            (false, false) => Self::Tie,
        }
    }
}

fn find_lowest_mana(
    player: &Character,
    boss: &Character,
    hard: bool,
    mana_spent: i32,
    mut lowest_mana_spent: Option<i32>,
) -> Option<i32> {
    let possible_spells = player.possible_spells(&boss);
    if possible_spells.len() == 0 {
        return None;
    }

    for spell in possible_spells {
        let mut player = player.clone();
        let mut boss = boss.clone();
        let mana_spent = mana_spent + spell.cost();

        if lowest_mana_spent.is_some() && mana_spent >= lowest_mana_spent.unwrap() {
            continue;
        }

        player.cast(&mut boss, spell, hard);

        match RoundOutcome::determine(&player, &boss) {
            RoundOutcome::StillFighting => {
                let boss_turn_outcome =
                    find_lowest_mana_boss_turn(&player, &boss, hard, mana_spent, lowest_mana_spent);
                if boss_turn_outcome.is_some() {
                    lowest_mana_spent = boss_turn_outcome;
                }
            }
            RoundOutcome::PlayerWon => {
                lowest_mana_spent = if lowest_mana_spent.is_some() {
                    cmp::min(Some(mana_spent), lowest_mana_spent)
                } else {
                    Some(mana_spent)
                };
            }
            _ => (),
        }
    }

    lowest_mana_spent
}

fn find_lowest_mana_boss_turn(
    player: &Character,
    boss: &Character,
    hard: bool,
    mana_spent: i32,
    lowest_mana_spent: Option<i32>,
) -> Option<i32> {
    let mut player = player.clone();
    let mut boss = boss.clone();

    boss.attack(&mut player);

    match RoundOutcome::determine(&player, &boss) {
        RoundOutcome::StillFighting => {
            find_lowest_mana(&player, &boss, hard, mana_spent, lowest_mana_spent)
        }
        RoundOutcome::PlayerWon => Some(mana_spent),
        _ => None,
    }
}

fn main() {
    let input = std::fs::read_to_string("input/2015/22.txt").expect("input should exist");

    let boss = Character::new_boss(&input);
    let player = Character::new_player();

    println!(
        "You need to spend at least {} mana to defeat the boss.",
        find_lowest_mana(&player, &boss, false, 0, None).unwrap()
    );
    println!(
        "If you play on hard, you need at least {} mana to defeat teh boss.",
        find_lowest_mana(&player, &boss, true, 0, None).unwrap()
    );
}
