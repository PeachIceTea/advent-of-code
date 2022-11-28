#[derive(Clone)]
struct Lightshow {
    lights: Vec<Vec<bool>>,
}

impl Lightshow {
    fn new(input: &str) -> Lightshow {
        let mut lights = vec![vec![false; 100]; 100];
        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                if c == '#' {
                    lights[x][y] = true;
                }
            }
        }
        Lightshow { lights }
    }

    fn simulate(&mut self, iterations: i32, stuck_corners: bool) {
        if stuck_corners {
            self.turn_corners_on();
        }

        for _ in 1..=iterations {
            let mut output = vec![vec![false; 100]; 100];
            for (x, row) in self.lights.iter().enumerate() {
                for (y, light) in row.iter().enumerate() {
                    let mut neighbors = 0;

                    for x_offset in -1..=1 {
                        for y_offset in -1..=1 {
                            if y_offset == 0 && x_offset == 0 {
                                continue;
                            }

                            let x = x as isize + x_offset;
                            let y = y as isize + y_offset;
                            if x < 0 || y < 0 || y >= 100 || x >= 100 {
                                continue;
                            }

                            if self.lights[x as usize][y as usize] {
                                neighbors += 1;
                            }
                        }
                    }

                    output[x][y] = (*light && (neighbors == 2 || neighbors == 3))
                        || (!light && neighbors == 3);
                }
            }

            self.lights = output;
            if stuck_corners {
                self.turn_corners_on();
            }
        }
    }

    fn turn_corners_on(&mut self) {
        self.lights[0][0] = true;
        self.lights[0][99] = true;
        self.lights[99][99] = true;
        self.lights[99][0] = true;
    }

    fn light_count(&self) -> i32 {
        let mut light_count = 0;
        for row in &self.lights {
            for val in row {
                if *val {
                    light_count += 1;
                }
            }
        }
        light_count
    }
}

fn main() {
    let input = std::fs::read_to_string("input/18.txt").expect("input should exist");

    let mut lightshow = Lightshow::new(&input);
    let mut lightshow_stuck = lightshow.clone();
    lightshow.simulate(100, false);

    println!(
        "After 100 steps there are {} lights on.",
        lightshow.light_count()
    );

    lightshow_stuck.simulate(100, true);
    println!(
        "If the corners are stuck on then after 100 steps there are {} lights.",
        lightshow_stuck.light_count()
    );
}
