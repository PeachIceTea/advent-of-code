use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/01.txt").expect("input should exist");

    let mut loc: (i32, i32) = (0, 0);
    let mut direction: i8 = 0;

    let mut history = HashSet::new();
    let mut bunny_hq = None;
    for movement in input.split(", ") {
        if movement.starts_with("R") {
            direction = (direction + 1) % 4;
        } else {
            direction = (direction - 1).rem_euclid(4)
        }

        let steps: i32 = movement[1..movement.len()].parse().unwrap();
        for _ in 1..=steps {
            match direction {
                0 => loc.0 += 1,
                1 => loc.1 += 1,
                2 => loc.0 -= 1,
                3 => loc.1 -= 1,
                _ => (),
            }

            if bunny_hq.is_none() {
                if history.contains(&loc) {
                    bunny_hq = Some(loc);
                } else {
                    history.insert(loc);
                }
            }
        }
    }
    println!(
        "The instructions lead you {} blocks away.",
        loc.0.abs() + loc.1.abs()
    );
    let bunny_hq = bunny_hq.expect("at least one location should have been visited twice.");
    println!(
        "Bunny HQ is actually {} blocks away.",
        bunny_hq.0.abs() + bunny_hq.1.abs()
    );
}
