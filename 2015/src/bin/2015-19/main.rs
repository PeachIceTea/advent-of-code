use serde_scan::scan;
use std::collections::{HashMap, HashSet};

fn calculate_calibration(
    input: &String,
    replacements: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut variants = HashSet::new();
    for (key, val_arr) in replacements.into_iter() {
        for val in val_arr {
            let key_length = key.len();
            for (index, _) in input.rmatch_indices(key) {
                let mut variant = input.clone();
                variant.replace_range(index..index + key_length, val);
                variants.insert(variant);
            }
        }
    }

    variants
}

fn create_reverse_replacements_list(
    replacements: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut replacement_list = HashMap::new();
    for (key, vals) in replacements {
        for val in vals {
            replacement_list.insert(
                val.chars().rev().collect::<String>(),
                key.chars().rev().collect::<String>(),
            );
        }
    }
    replacement_list
}

// I gave up after about 10 hours of trying :(
// Based on this: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/
fn calculate_fabrication(input: &String, replacements: &HashMap<String, String>) -> i32 {
    let mut input = input.chars().rev().collect::<String>();

    let reg = regex::Regex::new(
        &replacements
            .iter()
            .map(|(a, _)| a.clone())
            .collect::<Vec<String>>()
            .join("|"),
    )
    .unwrap();

    let mut count = 0;
    while input != "e" {
        let m = reg.find(&input).expect("No match before 'e'.");
        input.replace_range(m.range(), replacements.get(m.as_str()).unwrap());
        count += 1;
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input/19.txt").expect("input should exist");
    let mut replacements = HashMap::new();

    let mut iter = input.lines().into_iter();
    for line in &mut iter {
        if line == "" {
            break;
        }

        let (key, val): (String, String) = scan!("{} => {}" <- line).unwrap();
        replacements.entry(key).or_insert_with(Vec::new).push(val);
    }
    let target_molecule = iter.next().unwrap().to_string();

    let calibration_variants = calculate_calibration(&target_molecule, &replacements);
    println!(
        "The calibration results in {} unique molecules.",
        calibration_variants.len()
    );

    let reversed_replacements = create_reverse_replacements_list(&replacements);
    let step = calculate_fabrication(&target_molecule, &reversed_replacements);

    println!("The medicine can be made in {:?} steps.", step);
}
