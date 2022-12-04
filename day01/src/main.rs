use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let matches = clap::App::new("calorie-counting")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds information on the calories carried by elves")
        .args_from_usage("<FILENAME>     'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let groups = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                line.parse::<i32>().unwrap().into()
            }
        })
        .group_by(|num| num.is_some());

    let mut inventories: Vec<i32> = groups
        .into_iter()
        .filter(|group| group.0)
        .map(|group| group.1.map(Option::unwrap).sum::<i32>())
        .collect();

    let idx = inventories.len() - 3;
    inventories.select_nth_unstable(idx);

    // First problem
    println!(
        "Max inventory carries {} calories",
        inventories[idx..].iter().max().unwrap()
    );

    // Second problem
    println!(
        "Max three inventories carry {} calories",
        inventories[idx..].iter().sum::<i32>()
    );
}
