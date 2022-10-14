use std::str::FromStr;

use serde_derive::Deserialize;
use serde_scan::scan;

#[derive(Deserialize)]
struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn surface_area(&self) -> i32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn smallest_side_area(&self) -> i32 {
        let mut smallest_area = self.length * self.width;
        for area in [self.width * self.height, self.height * self.length] {
            if area < smallest_area {
                smallest_area = area
            }
        }
        smallest_area
    }

    fn required_wrapping_paper(&self) -> i32 {
        self.surface_area() + self.smallest_side_area()
    }

    fn ribbon_for_wrapping(&self) -> i32 {
        let mut sides = [self.length, self.width, self.height];
        sides.sort_unstable();
        sides[0] * 2 + sides[1] * 2
    }

    fn ribbon_for_bow(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn required_ribbon_length(&self) -> i32 {
        self.ribbon_for_wrapping() + self.ribbon_for_bow()
    }
}

impl FromStr for Present {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Present = scan!("{}x{}x{}" <- s)?;
        Ok(p)
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day2/input.txt")
        .expect("input file should be at input/day2/input.txt");

    let mut total_paper_needed = 0;
    let mut total_ribbon_needed = 0;
    for (line_num, line) in input.lines().enumerate() {
        let pres = match Present::from_str(line) {
            Ok(val) => val,
            Err(err) => panic!("could not parse line {line_num}: {err}"),
        };

        // First part.
        total_paper_needed += pres.required_wrapping_paper();

        // Second part.
        total_ribbon_needed += pres.required_ribbon_length();
    }
    println!("The elves should order {total_paper_needed} square feet of wrapping paper and a total of {total_ribbon_needed} feet of ribbon.");
}
