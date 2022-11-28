use logos::Source;

fn main() {
    let input = std::fs::read_to_string("input/08.txt").expect("input should exist");

    let mut base_size = 0;
    let mut decoded_size = 0; // Part 1
    let mut encoded_size = 0; // Part 2
    for line in input.lines() {
        base_size += line.len();
        let mut char_iter = line.slice(1..line.len() - 1).unwrap().chars();
        encoded_size += 6; // Surrounding ", \ for original ", and original "
        while let Some(c) = char_iter.next() {
            if c == '\\' {
                let c = char_iter.next().unwrap();
                encoded_size += 2; // \ and original \
                match c {
                    '\\' | '"' => {
                        decoded_size += 1;
                        encoded_size += 2; // \ and original " or \
                    }
                    'x' => {
                        if !char_iter.next().unwrap().is_ascii_hexdigit()
                            || !char_iter.next().unwrap().is_ascii_hexdigit()
                        {
                            panic!("Found non-hexdigit char in ascii escape sequence: {}", c);
                        }
                        decoded_size += 1;
                        encoded_size += 3; // x and two hexdigits
                    }
                    _ => panic!("Found unexpected escape sequence: {}", c),
                }
            } else {
                decoded_size += 1;
                encoded_size += 1;
            }
        }
    }

    println!("The base strings have a total size of {}.", base_size);
    println!("If we decode the strings they have a total size of {}, {} characters smaller than the base.", decoded_size, base_size-decoded_size);
    println!("If we encode the strings they have a total size of {}, {} characters larger than the base.", encoded_size, encoded_size-base_size);
}
