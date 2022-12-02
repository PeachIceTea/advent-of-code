use itertools::Itertools;

enum Goal {
    Win,
    Tie,
    Loose,
}

impl Goal {
    fn from_input(input: &str) -> Self {
        match input {
            "X" => Self::Loose,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_input(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn select_shape_with_goal(&self, goal: &Goal) -> Shape {
        match goal {
            Goal::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
            Goal::Tie => self.clone(),
            Goal::Loose => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            },
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, opponent_move: &Shape) -> i32 {
        if *opponent_move == *self {
            return 3;
        }

        match (opponent_move, self) {
            (Shape::Paper, Shape::Rock)
            | (Shape::Rock, Shape::Scissors)
            | (Shape::Scissors, Shape::Paper) => 0,
            _ => 6,
        }
    }
}

fn both_inputs_shapes(input: &str) -> i32 {
    let mut score = 0;
    for game in input.lines() {
        let (opponent_move, player_move) = game
            .split(" ")
            .map(|m| Shape::from_input(m))
            .collect_tuple()
            .unwrap();

        score += player_move.score() + Shape::play(&opponent_move, &player_move);
    }
    score
}

fn second_input_goal(input: &str) -> i32 {
    let mut score = 0;
    for game in input.lines() {
        let game = game.split(" ").collect::<Vec<&str>>();
        let (opponent_move, player_goal) = game.iter().collect_tuple().unwrap();

        let opponent_move = Shape::from_input(&opponent_move);
        let player_goal = Goal::from_input(&player_goal);
        let player_move = opponent_move.select_shape_with_goal(&player_goal);
        score += player_move.score() + player_move.play(&opponent_move);
    }
    score
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();

    println!(
        "If I were to follow the strategy guide and the second column refers to my move, my score would be {}.",
        both_inputs_shapes(&input)
    );
    println!(
        "If I were to follow the strategy guide and the second column refers to the outcome, my score would be {}.",
        second_input_goal(&input)
    );
}
