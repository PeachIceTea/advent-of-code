use std::collections::HashMap;

fn house_with_presents_count(instructions: &str, robo_santa: bool) -> usize {
    let mut visit_log = HashMap::new();

    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);

    // Log visit at 0,0.
    visit_log.insert(String::from("0,0"), 1);

    for (index, instruction) in instructions.char_indices() {
        let (x, y) = if index % 2 == 0 && robo_santa {
            (&mut robo_x, &mut robo_y)
        } else {
            (&mut santa_x, &mut santa_y)
        };

        match instruction {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => continue,
        }

        visit_log
            .entry(format!("{},{}", *x, *y))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    visit_log.len()
}

fn main() {
    let input = std::fs::read_to_string("input/2015/03.txt")
        .expect("input file should be at input/day3/input.txt");

    println!(
        "During the first year {} houses will receive presents and with Robo-Santa's help in the second year {} houses will get presents.",
        house_with_presents_count(input.as_str(), false),
        house_with_presents_count(input.as_str(), true),
    );
}
