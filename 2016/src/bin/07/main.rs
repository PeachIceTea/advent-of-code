use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/07.txt").expect("input should exist");

    let mut tls_count = 0;
    let mut ssl_count = 0;
    for line in input.lines() {
        let mut abba_queue = VecDeque::new();

        let mut in_brackets = false;

        let mut abba_found = false;
        let mut abba_in_brackets = false;

        let mut aba = Vec::new();
        let mut bab = Vec::new();

        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                in_brackets = true;
            } else if c == ']' {
                in_brackets = false;
            }

            abba_queue.push_back(c);
            if abba_queue.len() < 4 {
                continue;
            }
            if abba_queue.len() > 4 {
                abba_queue.pop_front();
            }

            let (a1, b1, b2, a2) = abba_queue.iter().collect_tuple().unwrap();

            // TLS
            if (a1, b1) == (a2, b2) && a1 != b1 {
                abba_found = true;
                abba_in_brackets = abba_in_brackets || in_brackets;
            }

            // SSL
            if i == 3 {
                if a1 == b2 && a1 != b1 {
                    if in_brackets && a2 != &'[' {
                        bab.push((*b1, *a1));
                    } else {
                        aba.push((*a1, *b1));
                    }
                }
            }

            if b1 == a2 && b1 != b2 {
                if in_brackets {
                    bab.push((*b2, *b1));
                } else {
                    aba.push((*b1, *b2));
                }
            }
        }

        if abba_found && !abba_in_brackets {
            tls_count += 1;
        }

        'outer: for (a1, b1) in &aba {
            for (a2, b2) in &bab {
                if a1 == a2 && b1 == b2 {
                    ssl_count += 1;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", tls_count);
    println!("{}", ssl_count);
}
