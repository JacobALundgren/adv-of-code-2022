use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

enum Outcome {
    Loss,
    Draw,
    Win,
}

fn outcome(game: &str) -> Outcome {
    let opponent_shape = game.as_bytes()[0] + (b'X' - b'A');
    let shape = game.as_bytes()[2];
    match (shape + 3 - opponent_shape) % 3 {
        0 => Outcome::Draw,
        1 => Outcome::Win,
        2 => Outcome::Loss,
        _ => panic!("Rules of modulus broken"),
    }
}

fn outcome_points(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn shape_points(game: &str) -> u32 {
    (game.as_bytes()[2] as u32) - ('W' as u32)
}

fn adjust_for_desired_outcome(game: &str) -> String {
    let opponent_shape = game.as_bytes()[0] + (b'X' - b'A');
    let desired_outcome = match game.as_bytes()[2] as char {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Invalid strategy"),
    };

    let mut desired_shape = opponent_shape
        + match desired_outcome {
            Outcome::Loss => 2,
            Outcome::Draw => 0,
            Outcome::Win => 1,
        };
    if desired_shape > b'Z' {
        desired_shape -= 3;
    }

    format!("{} {}", game.as_bytes()[0] as char, desired_shape as char)
}

fn main() {
    let matches = clap::App::new("rock-paper-scissors")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Evaluates rock-paper-scissors strategies")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    // First problem
    let score_total = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            outcome_points(outcome(&line)) + shape_points(&line)
        })
        .sum::<u32>();
    println!("Total score obtained first interpretation: {score_total}");

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    // Second problem
    let second_score_total = file
        .lines()
        .map(|line| {
            let line = adjust_for_desired_outcome(&line.unwrap());
            outcome_points(outcome(&line)) + shape_points(&line)
        })
        .sum::<u32>();
    println!("Total score obtained second interpretation: {second_score_total}");
}
