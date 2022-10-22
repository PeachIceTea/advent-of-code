fn create_zero_string(num_of_zeroes: i32) -> String {
    let mut s = String::new();
    for _ in 0..num_of_zeroes {
        s.push('0');
    }
    s
}

fn find_advent_coin_num(secret_key: &str, num_of_zeroes: i32) -> (String, i32) {
    let starting_zeroes = create_zero_string(num_of_zeroes);
    let mut current_num = 1;
    loop {
        let hash = format!("{:x}", md5::compute(format!("{secret_key}{current_num}")));
        if hash.starts_with(starting_zeroes.as_str()) {
            break (hash, current_num);
        }
        current_num += 1;
    }
}

fn main() {
    let secret_key = std::fs::read_to_string("input/2015/04.txt")
        .expect("input file should be at input/day4/input.txt");

    let (hash5, num5) = find_advent_coin_num(secret_key.as_str(), 5);
    let (hash6, num6) = find_advent_coin_num(secret_key.as_str(), 6);
    println!("For the secret \"{}\" the first number to create a hash with 5 leading zeroes is {} creating the hash {} and with 6 leading zeroes the number is {} creating the hash {}.", secret_key, num5, hash5, num6, hash6);
}
