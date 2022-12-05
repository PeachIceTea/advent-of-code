use itertools::Itertools;
use serde_scan::scan;

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();

    let (initial_arragement, moves) = input.split("\n\n").collect_tuple().unwrap();

    let mut stack_9000 = Vec::new();
    for (i, line) in initial_arragement.lines().rev().enumerate() {
        if i == 0 {
            for _ in line.split("  ").map(|t| t.trim()) {
                stack_9000.push(Vec::new());
            }
        }

        for i in 0..stack_9000.len() {
            let b = line.chars().nth(i * 4 + 1).unwrap();
            if !b.is_whitespace() {
                stack_9000[i].push(b);
            }
        }
    }
    let mut stack_9001 = stack_9000.clone();

    for line in moves.lines() {
        let (amount, from, to): (usize, usize, usize) =
            scan!("move {} from {} to {}" <- line).unwrap();

        for _ in 0..amount {
            let item = stack_9000[from - 1].pop().unwrap();
            stack_9000[to - 1].push(item);
        }

        let split_off_point = stack_9001[from - 1].len() - amount;
        let items = stack_9001[from - 1].split_off(split_off_point);
        stack_9001[to - 1].extend(items);
    }

    let result_9000 = stack_9000
        .iter()
        .map(|l| l.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");
    let result_9001 = stack_9001
        .iter()
        .map(|l| l.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    println!(
        "If we used the CrateMover 9000 the crates on the top would have been: {}",
        result_9000
    );
    println!(
        "Since we used the CrateMove 9001  the creates on the top were actually: {}",
        result_9001
    );
}
