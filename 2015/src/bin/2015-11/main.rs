use logos::Source; // for &[u8].slice()

fn main() {
    let input = std::fs::read_to_string("input/11.txt").expect("input should exist");

    let first_password = find_next_password(&input);
    let second_password = find_next_password(&first_password);

    println!(
        "Santa's next two passwords should be {} and {}.",
        first_password, second_password
    );
}

fn find_next_password(input: &str) -> String {
    let mut new_password = increment(&input);
    while !verify_string(&new_password) {
        new_password = increment(&new_password);
    }
    new_password
}

fn increment(input: &str) -> String {
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
        last_copied = input.len() - i - 1;

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

    let mut letter_sequence_count = 0;
    let mut letter_pair_count = 0;

    let mut prev_letter = ' ';
    let mut prev_prev_letter = ' ';
    for letter in input.chars() {
        if MISTAKEABLE_CHARACTERS.contains(&letter) {
            return false;
        }

        if letter as u8 - 1 == prev_letter as u8 && prev_prev_letter as u8 + 1 == prev_letter as u8
        {
            letter_sequence_count += 1;
        }

        if letter == prev_letter && letter != prev_prev_letter {
            letter_pair_count += 1;
        }

        prev_prev_letter = prev_letter;
        prev_letter = letter;
    }

    letter_sequence_count >= 1 && letter_pair_count >= 2
}
