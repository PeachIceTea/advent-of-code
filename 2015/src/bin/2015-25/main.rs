use serde_scan::scan;

fn main() {
    let input = std::fs::read_to_string("input/25.txt").expect("input should exist");
    let (target_row, target_column): (i32, i32) = {
        let input = input.as_str();
        scan!("To continue, please consult the code grid in the manual.  Enter the code at row {}, column {}." <- input).unwrap()
    };

    let mut code: i64 = 20151125;
    let mut row = 1;
    'outer: loop {
        row += 1;
        let mut column = 0;
        for row in (1..=row).rev() {
            column += 1;
            code = (code * 252533) % 33554393;

            if row == target_row && column == target_column {
                break 'outer;
            }
        }
    }

    println!("The code at {}x{} is {}.", target_row, target_column, code);
}
