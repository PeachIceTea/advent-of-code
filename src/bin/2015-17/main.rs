use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input/2015/17.txt").expect("input should exist");

    let mut sizes: Vec<i32> = Vec::new();
    for line in input.lines() {
        sizes.push(line.parse().unwrap());
    }

    let combinations = find_eggnog_store_options(&sizes, 150, 0, &mut vec![]);
    println!(
        "There are {} different combinations of containers to store the eggnog.",
        combinations.len(),
    );

    let mut shortest_combination_size: Option<usize> = None;
    let mut shortest_combination_count = 0;
    for combination in &combinations {
        let len = combination.len();
        if shortest_combination_size.is_some() {
            match len.cmp(&shortest_combination_size.unwrap()) {
                Ordering::Equal => shortest_combination_count += 1,
                Ordering::Greater => continue,
                Ordering::Less => {
                    shortest_combination_size = Some(len);
                    shortest_combination_count = 1;
                }
            }
        } else {
            shortest_combination_size = Some(len);
            shortest_combination_count = 1;
        }
    }
    println!("The minimum amount of containers needed to store the eggnog is {} and there are {} different ways to use that many.", shortest_combination_size.unwrap(), shortest_combination_count);
}

fn find_eggnog_store_options(
    input: &Vec<i32>,
    eggnog: i32,
    start: usize,
    used_containers: &mut Vec<i32>,
) -> Vec<Vec<i32>> {
    let mut combinations = Vec::new();
    for i in start..input.len() {
        let mut used_containers = used_containers.clone();
        used_containers.push(*input.get(i).unwrap());
        let overall_contained: i32 = used_containers.iter().sum();
        match overall_contained.cmp(&eggnog) {
            Ordering::Less => {
                combinations.append(&mut find_eggnog_store_options(
                    input,
                    eggnog,
                    i + 1,
                    &mut used_containers,
                ));
            }
            Ordering::Greater => continue,
            Ordering::Equal => combinations.push(used_containers.clone()),
        }
    }
    combinations
}
