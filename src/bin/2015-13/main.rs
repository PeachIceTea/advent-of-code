use std::collections::HashMap;

use itertools::Itertools;
use serde_scan::scan;

fn main() {
    let input = std::fs::read_to_string("input/2015/13.txt").expect("input should exist");

    // A clone of the HashMap setup in 2015-09.
    let mut people: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let (person, loose_or_gain, happiness, neighbor): (String, String, i32, String) =
            scan!("{} would {} {} happiness units by sitting next to {}." <- line).unwrap();
        let happiness = match loose_or_gain.as_str() {
            "gain" => happiness,
            "lose" => happiness * -1,
            _ => panic!("Unexpected happiness change type"),
        };
        if let Some(e) = people.get_mut(&person) {
            e.insert(neighbor.clone(), happiness);
        } else {
            people.insert(
                person.clone(),
                HashMap::from([(neighbor.clone(), happiness)]),
            );
        }
    }

    let (max_happiness_seating, max_happiness) = calculate_happiness(&people);

    // Add "Me" to HashMap.
    let mut me_neighbors = HashMap::new();
    for (name, neighbors) in &mut people {
        neighbors.insert(String::from("Me"), 0);
        me_neighbors.insert(name.clone(), 0);
    }
    people.insert(String::from("Me"), me_neighbors);
    let (max_happiness_seating_with_me, max_happiness_with_me) = calculate_happiness(&people);

    println!(
        "The optimal seating would be {:?} with a change in happiness of {}.",
        max_happiness_seating, max_happiness
    );
    println!(
        "Once I am also seated the seating would be {:?} with a change in happiness of {}.",
        max_happiness_seating_with_me, max_happiness_with_me
    );
}

fn calculate_happiness(people: &HashMap<String, HashMap<String, i32>>) -> (Vec<String>, i32) {
    let person_count = people.len();
    let mut max_happiness = 0;
    let mut max_happiness_seating: Vec<String> = Vec::new();
    for seating in people.into_iter().permutations(person_count) {
        let mut happiness = 0;
        let mut seating_names = Vec::new();

        for ((p1_name, p1_neighbors), (p2_name, p2_neighbors)) in
            seating.into_iter().tuple_windows()
        {
            happiness += p1_neighbors.get(p2_name).unwrap();
            happiness += p2_neighbors.get(p1_name).unwrap();
            if seating_names.len() == 0 {
                seating_names.push(p1_name.clone());
            }
            seating_names.push(p2_name.clone());
        }

        let first_person = seating_names.first().unwrap();
        let last_person = seating_names.last().unwrap();
        happiness += people.get(first_person).unwrap().get(last_person).unwrap();
        happiness += people.get(last_person).unwrap().get(first_person).unwrap();

        if happiness > max_happiness {
            max_happiness = happiness;
            max_happiness_seating = seating_names;
        }
    }

    (max_happiness_seating, max_happiness)
}
