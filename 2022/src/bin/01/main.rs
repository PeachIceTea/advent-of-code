fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();

    let mut elves = Vec::new();
    let mut current_elf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf.iter().sum::<i32>());
            current_elf = Vec::new();
            continue;
        }

        current_elf.push(line.parse().unwrap());
    }
    elves.sort();

    println!(
        "The elf with the most calories is carrying {} calories.",
        elves.last().unwrap()
    );

    println!(
        "The top three elves carry {} calories.",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    );
}
