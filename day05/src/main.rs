use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let matches = clap::App::new("supply-stacks")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Simulates crate movement")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let mut lines = file.lines().map(|line| line.unwrap());

    let mut piles = Vec::<Vec<u8>>::new();
    for line in lines.by_ref().take_while(|line| line.as_bytes()[1] != b'1') {
        let num_piles = (line.as_bytes().len() + 1) / 4;
        piles.resize(num_piles, Default::default());
        let boxes = line.as_bytes().iter().skip(1).step_by(4);
        let boxes_in_piles = boxes.zip(piles.iter_mut()).filter(|(&id, _)| id != b' ');
        for (&id, pile) in boxes_in_piles {
            pile.push(id);
        }
    }
    for pile in piles.iter_mut() {
        pile.reverse();
    }
    let lines = lines.skip(1);

    let parse_operation = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let operations: Vec<_> = lines
        .map(|line| {
            let captures = parse_operation.captures(&line).unwrap();
            let count: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            (count, from, to)
        })
        .collect();

    {
        let mut piles = piles.clone();
        for (count, from, to) in operations.iter() {
            for _ in 0..*count {
                if let Some(c) = piles[*from].pop() {
                    piles[*to].push(c);
                }
            }
        }

        let top_crates = piles
            .iter()
            .fold(String::with_capacity(piles.len()), |mut acc, pile| {
                acc.push((*pile.last().unwrap_or(&b' ')).into());
                acc
            });
        println!("First task top crates: {top_crates}");
    }

    for (count, from, to) in operations {
        let (from, to) = {
            let mut it = piles.iter_mut();
            let first = it.nth(std::cmp::min(from, to)).unwrap();
            let second = it
                .nth(std::cmp::max(from, to) - std::cmp::min(from, to) - 1)
                .unwrap();
            if from < to {
                (first, second)
            } else {
                (second, first)
            }
        };
        let available = from.len();
        let begin = if available >= count {
            available - count
        } else {
            0
        };
        to.extend_from_slice(&from[begin..]);
        from.truncate(begin);
    }
    let top_crates = piles
        .iter()
        .fold(String::with_capacity(piles.len()), |mut acc, pile| {
            acc.push((*pile.last().unwrap_or(&b' ')).into());
            acc
        });
    println!("Second task top crates: {top_crates}");
}
