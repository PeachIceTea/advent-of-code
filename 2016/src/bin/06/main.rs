use std::collections::HashMap;

use itertools::Itertools;

fn find_most_common(hm: &HashMap<char, i32>) -> char {
    let mut most_common_count = 0;
    let mut most_common_char = ' ';
    for (ch, count) in hm {
        if count > &most_common_count {
            most_common_count = *count;
            most_common_char = *ch;
        }
    }
    most_common_char
}

fn find_least_common(hm: &HashMap<char, i32>) -> char {
    let mut least_common_count = -1;
    let mut least_common_char = ' ';
    for (ch, count) in hm {
        if least_common_count == -1 || count < &least_common_count {
            least_common_count = *count;
            least_common_char = *ch;
        }
    }
    least_common_char
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").expect("input should exist");

    let mut counters = HashMap::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            *counters
                .entry(i)
                .or_insert(HashMap::new())
                .entry(c)
                .or_insert(0) += 1;
        }
    }

    let mut most_common_message = String::new();
    let mut least_common_message = String::new();
    for (_, counter) in counters.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
        most_common_message.push(find_most_common(&counter));
        least_common_message.push(find_least_common(&counter));
    }
    println!("The message using normal repetition code is {}.\nUsing the modified version the password is {}.", most_common_message, least_common_message);
}
