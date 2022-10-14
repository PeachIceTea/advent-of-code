fn main() {
    let input = std::fs::read_to_string("input/day1/input.txt")
        .expect("input file should be at input/day1/input.txt");

    // First part.
    println!(
        "Santa should go to floor {}",
        input.matches("(").count() - input.matches(")").count()
    );

    // Second part.
    let mut current_floor = 0;
    for (index, instruction) in input.char_indices() {
        match instruction {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }

        if current_floor == -1 {
            println!("Santa enters the basement in position {}", index + 1);
            break;
        }
    }
}
