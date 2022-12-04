use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_range(text: &str) -> (u32, u32) {
    let (l, r) = text.split_once('-').unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn parse_pair(text: &str) -> ((u32, u32), (u32, u32)) {
    let (l, r) = text.split_once(',').unwrap();
    (parse_range(l), parse_range(r))
}

fn either_is_subset(l: &(u32, u32), r: &(u32, u32)) -> bool {
    ((l.0 <= r.0) && (l.1 >= r.1)) || ((r.0 <= l.0) && (r.1 >= l.1))
}

fn ranges_overlap(l: &(u32, u32), r: &(u32, u32)) -> bool {
    ((l.0 <= r.0) && (l.1 >= r.0)) || ((r.0 <= l.0) && (r.1 >= l.0))
}

fn main() {
    let matches = clap::App::new("camp-cleanup")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Evaluates cleaning assignments")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let count = file
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_pair(&line))
        .filter(|(l, r)| either_is_subset(l, r))
        .count();

    println!("Number of pairs where either is a subset of the other: {count}");

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let count = file
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_pair(&line))
        .filter(|(l, r)| ranges_overlap(l, r))
        .count();

    println!("Number of pairs where the ranges overlap: {count}");
}
