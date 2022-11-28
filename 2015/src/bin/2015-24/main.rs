use itertools::Itertools;

fn quantum_entanglement(v: &Vec<&i32>) -> i64 {
    let mut qe = *v[0] as i64;
    for i in v.iter().skip(1) {
        qe *= (**i) as i64;
    }
    qe
}

fn balance_sleigh(input: &Vec<i32>, compartments: i32) -> i64 {
    let balance_weight: i32 = input.iter().sum::<i32>() / compartments;
    let mut lowest_qe = -1;
    for k in 1..=input.len() {
        for v in input.iter().combinations(k) {
            let sum: i32 = v.iter().map(|v| *v).sum();

            // Assumes other compartments can be filled evenly with the remaining inputs. Which I am
            // not sure is actually true for all inputs.
            if sum != balance_weight {
                continue;
            }

            let qe = quantum_entanglement(&v);
            if lowest_qe == -1 || qe < lowest_qe {
                lowest_qe = qe;
            }
        }

        if lowest_qe != -1 {
            break;
        }
    }
    lowest_qe
}

fn main() {
    let input: Vec<i32> = std::fs::read_to_string("input/24.txt")
        .expect("input should exist")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!(
        "The quantum entanglement of the ideal configuration is {}.",
        balance_sleigh(&input, 3)
    );
    println!(
        "With the trunk also filled the quantum entanglement for the ideal configuration is {}.",
        balance_sleigh(&input, 4)
    );
}
