use itertools::Itertools;

fn parse_line(input: &str) -> (i32, i32, i32) {
    input
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn is_valid_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn listed_as_rows(input: &str) -> i32 {
    let mut valid_triangles = 0;
    for line in input.lines() {
        let (a, b, c) = parse_line(line);
        if is_valid_triangle(a, b, c) {
            valid_triangles += 1;
        }
    }
    valid_triangles
}

fn listed_as_columns(input: &str) -> i32 {
    let mut valid_triangles = 0;
    for (l1, l2, l3) in input.lines().tuples() {
        let (l1_a, l1_b, l1_c) = parse_line(l1);
        let (l2_a, l2_b, l2_c) = parse_line(l2);
        let (l3_a, l3_b, l3_c) = parse_line(l3);
        if is_valid_triangle(l1_a, l2_a, l3_a) {
            valid_triangles += 1;
        }
        if is_valid_triangle(l1_b, l2_b, l3_b) {
            valid_triangles += 1;
        }
        if is_valid_triangle(l1_c, l2_c, l3_c) {
            valid_triangles += 1;
        }
    }
    valid_triangles
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").expect("input should exist");

    println!(
        "There are {} valid triangles in the specifications.",
        listed_as_rows(&input)
    );
    println!(
        "If using columns instead to define triangles, {} of them are valid.",
        listed_as_columns(&input)
    );
}
