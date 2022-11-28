use std::collections::HashMap;

use itertools::Itertools;
use serde_scan::scan;

fn main() {
    let input = std::fs::read_to_string("input/09.txt").expect("input should exist");

    // Parse file.
    let mut routes: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let (location, destination, distance): (String, String, i32) =
            scan!("{} to {} = {}" <- line).unwrap();

        if let Some(e) = routes.get_mut(&location) {
            e.insert(destination.clone(), distance);
        } else {
            routes.insert(
                location.clone(),
                HashMap::from([(destination.clone(), distance)]),
            );
        }

        if let Some(e) = routes.get_mut(&destination) {
            e.insert(location, distance);
        } else {
            routes.insert(destination, HashMap::from([(location, distance)]));
        }
    }

    let loc_count = routes.len();
    // Part 1
    let mut shortest_path_length = -1;
    let mut shortest_path_route: Vec<String> = Vec::new();

    let mut longest_path_length = -1;
    let mut longest_path_route: Vec<String> = Vec::new();
    for path in routes.into_iter().permutations(loc_count) {
        let mut route: Vec<String> = Vec::new();
        let mut length = 0;

        for (loc, dest) in path.into_iter().tuple_windows() {
            length += loc.1.get(&dest.0).unwrap();
            if route.len() == 0 {
                route.push(loc.0.clone());
            }
            route.push(dest.0.clone());
        }

        if shortest_path_length == -1 || shortest_path_length > length {
            shortest_path_length = length;
            shortest_path_route = route.clone();
        }

        if longest_path_length == -1 || longest_path_length < length {
            longest_path_length = length;
            longest_path_route = route;
        }
    }

    println!(
        "The shortest path is {} in length: {:?}.",
        shortest_path_length, shortest_path_route
    );

    println!(
        "The longest path is {} in length: {:?}.",
        longest_path_length, longest_path_route
    );
}
