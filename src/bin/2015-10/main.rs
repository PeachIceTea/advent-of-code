fn main() {
    let input = std::fs::read_to_string("input/2015/10.txt").expect("input should exist");

    let mut last_output = String::from(input);
    for i in 1..=50 {
        let mut current_output = String::new();
        let mut iter = last_output.chars().peekable();

        while let Some(c) = iter.next() {
            let mut count = 1;
            while iter.peek().is_some() && *iter.peek().unwrap() == c {
                iter.next();
                count += 1;
            }
            current_output.push_str(count.to_string().as_str());
            current_output.push(c);
        }
        last_output = current_output;

        if i == 40 || i == 50 {
            println!(
                "After {} iterations the result is {} characters long.",
                i,
                last_output.len()
            );
        }
    }
}
