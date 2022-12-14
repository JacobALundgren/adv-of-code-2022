use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const NUM_CHARS: usize = (b'z' - b'a' + 1) as usize;

struct Tracker {
    counts: [u32; NUM_CHARS],
    num_duplicated: u8,
}

impl Tracker {
    fn new<'a, I>(vals: I) -> Self
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut tracker = Tracker {
            counts: [0; NUM_CHARS],
            num_duplicated: 0,
        };

        for c in vals {
            tracker.push(*c);
        }
        tracker
    }

    fn push(&mut self, c: u8) {
        let entry = &mut self.counts[(c - b'a') as usize];
        self.num_duplicated += (*entry == 1) as u8;
        *entry += 1;
    }

    fn pop(&mut self, c: u8) {
        let entry = &mut self.counts[(c - b'a') as usize];
        *entry -= 1;
        self.num_duplicated -= (*entry == 1) as u8;
    }

    fn all_unique(&self) -> bool {
        self.num_duplicated == 0
    }
}

fn find_signature_position(input: &String, signature_length: usize) -> usize {
    let mut tracker = Tracker::new(input.as_bytes().iter().take(signature_length));

    let pos = signature_length
        + input
            .as_bytes()
            .iter()
            .skip(signature_length)
            .zip(input.as_bytes().iter())
            .position(|(&add, &rem)| {
                if tracker.all_unique() {
                    return true;
                }
                tracker.pop(rem);
                tracker.push(add);
                false
            })
            .unwrap_or(input.len() - signature_length);
    assert!(tracker.all_unique());

    pos
}

fn main() {
    let matches = clap::App::new("tuning-trouble")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Analyzes communication signals")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Unable to open {display}: {why}"),
        Ok(file) => BufReader::new(file),
    };

    let line = file.lines().next().unwrap().unwrap();

    println!(
        "Last four are unique after {} elements have been processed",
        find_signature_position(&line, 4)
    );
    println!(
        "Last fourteen are unique after {} elements have been processed",
        find_signature_position(&line, 14)
    );
}
