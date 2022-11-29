use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

fn decrypt(input: &str, id: i32) -> String {
    let input: Vec<&str> = input[0..input.find('[').unwrap()].split("-").collect();

    let words = &input[0..input.len() - 2];
    let shift_by: u8 = (id % 26) as u8;

    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }
        for mut c in word.bytes() {
            c += shift_by;
            if c > b'z' {
                c -= 26;
            }
            if c < b'a' {
                c += 26;
            }

            result.push(c as char)
        }
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("input/04.txt").expect("input should exist");

    let mut sum = 0;
    'line: for line in input.lines() {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for name in line.split("-") {
            if name.contains("[") {
                let (id, checksum) = {
                    let (id, cs) = name.split("[").collect_tuple().unwrap();
                    (id.parse::<i32>().unwrap(), &cs[0..cs.len() - 1])
                };

                let mut sorted_count: Vec<_> = counter.iter().collect();
                sorted_count.sort_by(|a, b| {
                    let cmp = b.1.cmp(a.1);
                    if cmp == Ordering::Equal {
                        a.0.cmp(b.0)
                    } else {
                        cmp
                    }
                });
                let sorted_count: Vec<_> = sorted_count.iter().map(|tmp| tmp.0).collect();

                if decrypt(line, id) == "northpole object" {
                    println!(
                        "The northpole object is stored in the roome with the sector ID {}.",
                        id
                    );
                }
                for (i, c) in checksum.chars().enumerate() {
                    if c != *sorted_count[i] {
                        continue 'line;
                    }
                }

                sum += id;
                break;
            }

            for c in name.chars() {
                *counter.entry(c).or_insert(0) += 1;
            }
        }
    }
    println!("The sum of all the real room's sector IDs is {}.", sum);
}
