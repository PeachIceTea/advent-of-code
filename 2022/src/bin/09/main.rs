use std::collections::HashSet;

use serde_scan::scan;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }

    fn add(mut self, (x, y): (i32, i32)) -> Self {
        self.x += x;
        self.y += y;
        return self;
    }
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();

    let mut head = Point::from((0, 0));
    let mut tail = Point::from((0, 0));
    let mut visited = HashSet::from([Point::from((0, 0))]);
    for line in input.lines() {
        let (direction, step): (char, i32) = scan!("{} {}" <- line).unwrap();
        'step: for _ in 0..step {
            match direction {
                'U' => head.x += 1,
                'D' => head.x -= 1,
                'L' => head.y -= 1,
                'R' => head.y += 1,
                _ => panic!(),
            }

            for check_x in -1..=1 {
                for check_y in -1..=1 {
                    if head == tail.clone().add((check_x, check_y)) {
                        continue 'step;
                    }
                }
            }

            let x_diff = tail.x.abs_diff(head.x);
            let y_diff = tail.y.abs_diff(head.y);

            let tails_is_above = tail.y < head.y;
            let tails_is_left = tail.x < head.x;

            if x_diff >= 1 && y_diff >= 1 {
                if x_diff == 1 {
                    if tails_is_left {
                        tail.x += 1;
                    } else {
                        tail.x -= 1;
                    }
                } else {
                    if tails_is_above {
                        tail.y += 1;
                    } else {
                        tail.y -= 1;
                    }
                }
            }

            let x_diff = tail.x.abs_diff(head.x);
            let y_diff = tail.y.abs_diff(head.y);

            if x_diff == 0 {
                if tails_is_above {
                    tail.y += 1;
                } else {
                    tail.y -= 1;
                }
            } else if y_diff == 0 {
                if tails_is_left {
                    tail.x += 1;
                } else {
                    tail.x -= 1;
                }
            }

            visited.insert(tail.clone());
        }
    }

    println!("{}", visited.len());
}
