use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/2015/05.txt")
        .expect("input file should be at input/day5/input.txt");

    // First part. I struggled with this quite a bit. This implementation is not as good as it could
    // be but I do not know what was wrong with my first one :<
    let vowel_list = ['a', 'e', 'i', 'o', 'u'];
    let naughty_list = ["ab", "cd", "pq", "xy"];

    let mut nice_string_count = 0;
    'line: for line in input.lines() {
        let mut vowels = 0;
        let mut has_double_letter = false;

        let mut prev_letter = ' ';
        for letter in line.chars() {
            let combo = format!("{prev_letter}{letter}");
            if naughty_list.contains(&combo.as_str()) {
                continue 'line;
            }

            if vowel_list.contains(&letter) {
                vowels += 1;
            }

            if letter == prev_letter {
                has_double_letter = true;
            }

            prev_letter = letter
        }

        if vowels >= 3 && has_double_letter {
            nice_string_count += 1;
        }
    }

    // Second part. Can you tell I kinda gave up on making this nice?
    let mut nice_string_count2 = 0;
    for line in input.lines() {
        let mut has_non_overlapping_pair = false;
        let mut has_repeating_letter_with_one_in_between = false;

        let mut pairs = HashMap::new();
        let mut prev_prev_letter = ' ';
        let mut prev_letter = ' ';
        for letter in line.chars() {
            if !(prev_letter == letter && prev_prev_letter == letter) {
                let combo = format!("{prev_letter}{letter}");
                pairs
                    .entry(combo)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            if prev_prev_letter == letter {
                has_repeating_letter_with_one_in_between = true;
            }

            prev_prev_letter = prev_letter;
            prev_letter = letter;
        }

        for pair_count in pairs.values() {
            if *pair_count >= 2 {
                has_non_overlapping_pair = true;
                break;
            }
        }

        if has_non_overlapping_pair && has_repeating_letter_with_one_in_between {
            nice_string_count2 += 1;
        }
    }

    println!("Using method 1 there are {nice_string_count} nice strings on Santa's text file and with method 2 there are {nice_string_count2} nice strings.");
}
