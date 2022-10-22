use std::{str::FromStr, vec};

use serde_scan::scan;

struct Point {
    x: usize,
    y: usize,
}

enum Action {
    On((Point, Point)),
    Off((Point, Point)),
    Toggle((Point, Point)),
}

impl FromStr for Action {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("toggle") {
            let (x1, y1, x2, y2): (usize, usize, usize, usize) =
                scan!("toggle {},{} through {},{}" <- s)?;
            Ok(Self::Toggle((
                Point { x: x1, y: y1 },
                Point { x: x2, y: y2 },
            )))
        } else {
            let (action, x1, y1, x2, y2): (String, usize, usize, usize, usize) =
                scan!("turn {} {},{} through {},{}" <- s)?;
            match action.as_str() {
                "on" => Ok(Self::On((Point { x: x1, y: y1 }, Point { x: x2, y: y2 }))),
                "off" => Ok(Self::Off((Point { x: x1, y: y1 }, Point { x: x2, y: y2 }))),
                _ => Err(format!("invalid action: {action}").into()),
            }
        }
    }
}

fn on_off(input: &str) -> i32 {
    let mut lights = vec![[false; 1000]; 1000];
    for (line_num, line) in input.lines().enumerate() {
        match Action::from_str(line) {
            Ok(instruction) => match instruction {
                Action::On((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            lights[x][y] = true;
                        }
                    }
                }
                Action::Off((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            lights[x][y] = false;
                        }
                    }
                }
                Action::Toggle((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            let light = &mut lights[x][y];
                            *light = !*light;
                        }
                    }
                }
            },
            Err(err) => panic!("could not parse line {line_num}: {err}"),
        }
    }

    let mut num_of_lights = 0;
    for line in lights {
        for light in line {
            if light {
                num_of_lights += 1;
            }
        }
    }
    num_of_lights
}

fn brightness(input: &str) -> u32 {
    let mut lights = vec![vec![0u8; 1000]; 1000];

    for (line_num, line) in input.lines().enumerate() {
        match Action::from_str(line) {
            Ok(instruction) => match instruction {
                Action::On((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            lights[x][y] += 1;
                        }
                    }
                }
                Action::Off((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            let light = &mut lights[x][y];
                            if *light > 0 {
                                *light -= 1;
                            }
                        }
                    }
                }
                Action::Toggle((start, end)) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            lights[x][y] += 2;
                        }
                    }
                }
            },
            Err(err) => panic!("could not parse line {line_num}: {err}"),
        }
    }
    let mut brightness: u32 = 0;
    for line in lights {
        for light in line {
            brightness += light as u32;
        }
    }
    brightness
}

fn main() {
    let input = std::fs::read_to_string("input/2015/06.txt")
        .expect("input file should be at input/day6/input.txt");

    println!(
        "After following Santa's instructions {} lights are turned on. After realizing your mistake and fixing it the total brightness of all lights is {}.",
        on_off(&input),
        brightness(&input),
    )
}
