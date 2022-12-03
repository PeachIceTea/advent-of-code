use itertools::Itertools;

fn priority(c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        c as i32 - 96
    } else {
        c as i32 - 64 + 26
    }
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();

    let mut duplicate_priorities = 0;
    let mut group = Vec::new();
    let mut group_priorities = 0;
    'line: for line in input.lines() {
        group.push(line);
        if group.len() == 3 {
            for c in group[0].chars() {
                if group[1].chars().contains(&c) && group[2].chars().contains(&c) {
                    group_priorities += priority(c);
                    break;
                }
            }
            group = Vec::new();
        }

        let front = &line[0..line.len() / 2];
        let back = &line[line.len() / 2..line.len()];
        for c in front.chars() {
            if back.chars().contains(&c) {
                duplicate_priorities += priority(c);
                continue 'line;
            }
        }
    }
    println!(
        "The sum of all priorities for duplicate items is {}.",
        duplicate_priorities
    );
    println!(
        "The sum of all priorities for group badges is {}.",
        group_priorities
    );
}
