use std::cmp;

use itertools::Itertools;

#[rustfmt::skip]
const NUMPAD: [[i32; 3]; 3] = [
    [1, 2, 3], 
    [4, 5, 6], 
    [7, 8, 9]
];
fn numpad_pos(i: i32) -> i32 {
    cmp::min(cmp::max(0, i), 2)
}

fn solve_numpad(input: &str) -> String {
    let mut code = Vec::new();

    let mut pos = (1, 1);
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => pos.1 = numpad_pos(pos.1 - 1),
                'D' => pos.1 = numpad_pos(pos.1 + 1),
                'R' => pos.0 = numpad_pos(pos.0 + 1),
                'L' => pos.0 = numpad_pos(pos.0 - 1),
                _ => (),
            }
        }
        code.push(NUMPAD[pos.1 as usize][pos.0 as usize]);
    }

    code.iter().join("")
}

#[rustfmt::skip]
const KEYPAD: [[Option<char>; 5]; 5] = [
    [None,      None,       Some('1'),  None,        None],
    [None,      Some('2'),  Some('3'),  Some('4'),   None],
    [Some('5'), Some('6'),  Some('7'),  Some('8'),   Some('9')],
    [None,      Some('A'),  Some('B'),  Some('C'),   None],
    [None,      None,       Some('D'),  None,        None],
];
fn valid_keypad_pos(pos: (i32, i32)) -> bool {
    if let Some(row) = KEYPAD.get(pos.1 as usize) {
        if let Some(val) = row.get(pos.0 as usize) {
            val.is_some()
        } else {
            false
        }
    } else {
        false
    }
}

fn solve_keypad(input: &str) -> String {
    let mut code = Vec::new();

    let mut pos = (0, 2);
    for line in input.lines() {
        for c in line.chars() {
            let mut new_pos = pos;
            match c {
                'U' => new_pos.1 = new_pos.1 - 1,
                'D' => new_pos.1 = new_pos.1 + 1,
                'R' => new_pos.0 = new_pos.0 + 1,
                'L' => new_pos.0 = new_pos.0 - 1,
                _ => (),
            }
            if valid_keypad_pos(new_pos) {
                pos = new_pos
            }
        }
        code.push(KEYPAD[pos.1 as usize][pos.0 as usize]);
    }

    code.iter().map(|i| i.unwrap()).join("")
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").expect("input should exist");

    println!(
        "The number pad I am imagining would have the code {}.",
        solve_numpad(&input)
    );

    println!("The actual keypad has the code {}.", solve_keypad(&input));
}
