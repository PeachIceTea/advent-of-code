use std::{cmp, collections::HashMap};

fn find_smallest_number_with_gifts(
    target: usize,
    gift_multiplier: usize,
    max_houses: Option<usize>,
) -> usize {
    let mut houses = HashMap::new();
    let max_house_number = target / 10;

    for elf in 1..=max_house_number {
        let range = if let Some(n) = max_houses {
            elf..=cmp::min(elf * n, max_house_number)
        } else {
            elf..=max_house_number
        };
        for house in (range).step_by(elf) {
            *houses.entry(house).or_insert(0) += elf * gift_multiplier;
        }
    }

    let mut lowest_house_number = max_house_number;
    for (house_number, gifts) in houses {
        if gifts >= target && house_number < lowest_house_number {
            lowest_house_number = house_number;
        }
    }

    lowest_house_number
}

fn main() {
    let input: usize = std::fs::read_to_string("input/20.txt")
        .expect("input should exist")
        .parse()
        .unwrap();
    println!("House #{} will recieve {} or more gifts when the elves visit an infinite amount of houses.", find_smallest_number_with_gifts(input, 10, None), input);
    println!(
        "House #{} will recieve {} or more gifts when each elf only visits 50 houses.",
        find_smallest_number_with_gifts(input, 11, Some(50)),
        input
    );
}
