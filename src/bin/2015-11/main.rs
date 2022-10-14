use itertools::Itertools;
use logos::Source;

fn main() {
    let input = std::fs::read_to_string("input/2015/11.txt").expect("input should exist");
    println!("{} -> {}", input, increment(&input));
}

fn increment(input: &str) -> String {
    if !input.is_ascii() {
        panic!("Input is not ascii");
    }

    let input = input.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut last_copied = 0;
    for (i, c) in input.into_iter().rev().enumerate() {
        let (c, wrapped) = {
            let c = c + 1;
            if c >= b'{' {
                ('a', true)
            } else {
                (c as char, false)
            }
        };

        output.insert(0, c);
        last_copied = input.len() - i;

        if !wrapped {
            break;
        }
    }

    output.insert_str(
        0,
        &std::str::from_utf8(input.slice(0..last_copied).unwrap()).unwrap(),
    );

    output
}

fn verify_string(input: &str) -> bool {
    const MISTAKEABLE_CHARACTERS: [char; 3] = ['i', 'o', 'l'];

    let mut letter_pair_count = 0;
    let mut prev_letter = ' ';
    let mut prev_prev_letter = ' ';
    for c in input.chars() {
        if MISTAKEABLE_CHARACTERS.contains(&c) {
            return false;
        }
    }

    true
}
