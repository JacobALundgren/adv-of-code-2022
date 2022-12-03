#![feature(iter_array_chunks)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn priority_value(item: u8) -> u8 {
    if (item as char).is_uppercase() {
        item - b'A' + 27
    } else {
        item - b'a' + 1
    }
}

fn main() {
    let matches = clap::App::new("rucksack-reorganization")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Evaluates rucksack organization")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let total_priority = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (l, r) = line.as_bytes().split_at(line.as_bytes().len() / 2);
            let l = l.iter().copied().collect::<HashSet<u8>>();
            let r = r.iter().copied().collect::<HashSet<u8>>();
            let common_item = l.intersection(&r).next().unwrap();
            priority_value(*common_item) as u32
        })
        .sum::<u32>();

    println!("Total priority of duplicate items: {total_priority}");

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let total_badge_priority = file
        .lines()
        .array_chunks::<3>()
        .map(|group| {
            let common_item = group
                .map(|line| {
                    line.unwrap()
                        .as_bytes()
                        .iter()
                        .copied()
                        .collect::<HashSet<u8>>()
                })
                .iter()
                .cloned()
                .reduce(|acc, item| acc.intersection(&item).copied().collect::<HashSet<u8>>())
                .unwrap()
                .iter()
                .copied()
                .next()
                .unwrap();
            priority_value(common_item) as u32
        })
        .sum::<u32>();

    println!("Total priority of badges: {total_badge_priority}");
}
