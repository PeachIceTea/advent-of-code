use serde_derive::Deserialize;
use serde_scan::scan;
use std::cmp::max;

#[derive(Deserialize)]
struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let input = std::fs::read_to_string("input/2015/15.txt").expect("input should exist");

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in input.lines() {
        ingredients.push(
            scan!("{}: capacity {}, durability {}, flavor {}, texture {}, calories {}" <- line)
                .unwrap(),
        );
    }

    let max_score = get_highest_score(&ingredients, &mut vec![], false);
    println!(
        "The cookie with the best ingredients has a score of {}.",
        max_score
    );

    let max_score_with_calorie_limit = get_highest_score(&ingredients, &mut vec![], true);
    println!(
        "If we set out to create a cookie with 500 calories then the best cookie has a score of {}.",
        max_score_with_calorie_limit
    );
}

fn get_highest_score(
    ingredients: &Vec<Ingredient>,
    amounts: &mut Vec<i32>,
    include_calories: bool,
) -> i32 {
    let mut highest_score = 0;

    let last_ingredient = ingredients.len() == (amounts.len() + 1);
    for s in 0..=(100 - amounts.iter().sum::<i32>()) {
        amounts.push(s);
        let score;
        if last_ingredient {
            score = get_score(ingredients, &amounts, include_calories);
        } else {
            score = get_highest_score(ingredients, amounts, include_calories);
        }
        if score > highest_score {
            highest_score = score;
        }
        amounts.pop().unwrap();
    }
    highest_score
}

fn get_score(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>, include_calories: bool) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (i, ingredient) in (&ingredients).into_iter().enumerate() {
        let amount = amounts.get(i).unwrap();
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
        calories += ingredient.calories * amount;
    }

    if include_calories && calories != 500 {
        0
    } else {
        max(0, capacity) * max(0, durability) * max(0, flavor) * max(0, texture)
    }
}
