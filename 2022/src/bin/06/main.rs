use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();

    let mut buffer = VecDeque::new();
    let mut sop_found = false;
    let mut som_found = false;
    for (i, c) in input.chars().enumerate() {
        buffer.push_back(c);
        if buffer.len() < 4 {
            continue;
        }
        if buffer.len() == 15 {
            buffer.pop_front();
        }

        if !sop_found && buffer.iter().rev().take(4).all_unique() {
            sop_found = true;
            println!("{} characters have to be processed until the first start-of-packet marker is found.", i+1);
        }

        if !som_found && buffer.iter().all_unique() {
            som_found = true;
            println!("{} characters have to be processed until the first start-of-message marker is found.", i+1)
        }

        if sop_found && som_found {
            break;
        }
    }
}
