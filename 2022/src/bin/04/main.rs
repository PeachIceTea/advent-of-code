use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();

    let mut fully_contained_pairs = 0;
    let mut any_overlap = 0;
    for line in input.lines() {
        let ((first_low, first_high), (second_low, second_high)): ((i32, i32), (i32, i32)) = line
            .split(",")
            .map(|t| {
                t.split("-")
                    .map(|t| t.parse::<i32>().unwrap())
                    .collect_tuple::<(i32, i32)>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        if first_low <= second_high && second_low <= first_high {
            any_overlap += 1;

            if first_low >= second_low && first_high <= second_high
                || second_low >= first_low && second_high <= first_high
            {
                fully_contained_pairs += 1;
            }
        }
    }

    println!(
        "There are {} assignment pairs that are fully contain the other.",
        fully_contained_pairs
    );
    println!(
        "There are {} assignment pairs that overlap at.",
        any_overlap
    );
}
