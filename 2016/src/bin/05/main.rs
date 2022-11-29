fn main() {
    let input = std::fs::read_to_string("input/05.txt").expect("input should exist");

    let mut first_password = String::with_capacity(8);
    let mut second_password = [None, None, None, None, None, None, None, None];

    let mut i = 0;
    while !(first_password.len() == 8
        && second_password
            .iter()
            .filter(|i| i.is_none())
            .collect::<Vec<_>>()
            .len()
            == 0)
    {
        let tmp = format!("{:x}", md5::compute(format!("{input}{i}")));
        if tmp.starts_with("00000") {
            let first_char = tmp.as_bytes()[5] as char;
            let second_char = tmp.as_bytes()[6] as char;

            if first_password.len() != 8 {
                first_password.push(first_char);
            }

            if let Ok(index) = first_char.to_string().parse::<usize>() {
                if index < second_password.len() && second_password[index].is_none() {
                    second_password[index] = Some(second_char);
                }
            }
        }
        i += 1;
    }

    let second_password = second_password.map(|i| i.unwrap().to_string()).join("");
    println!(
        "The password for the first door is {} and {} for the second door.",
        first_password, second_password
    );
}
