#![feature(hash_raw_entry)]
use std::{collections::HashMap, path::Path};

#[derive(Default, PartialEq)]
struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

struct DirectoryTraverse<'a> {
    iterators: Vec<Box<dyn Iterator<Item = (&'a str, &'a Directory)> + 'a>>,
}

impl<'a> Iterator for DirectoryTraverse<'a> {
    type Item = (&'a str, &'a Directory);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self
            .iterators
            .last_mut()
            .and_then(|iterator| iterator.next());
        while ret.is_none() && !self.iterators.is_empty() {
            self.iterators.pop();
            ret = self
                .iterators
                .last_mut()
                .and_then(|iterator| iterator.next());
        }
        if let Some((_, directory)) = ret {
            self.iterators.push(Box::new(
                directory
                    .subdirectories
                    .iter()
                    .map(|(name, directory)| (name.as_str(), directory)),
            ));
        }
        ret
    }
}

impl Directory {
    fn parse(input: &str) -> Self {
        let files = input
            .lines()
            .filter(|line| line.chars().next().unwrap().is_ascii_digit())
            .map(|line| {
                let (size, name) = line.split_once(char::is_whitespace).unwrap();
                (name.to_owned(), size.parse::<usize>().unwrap())
            });
        Directory {
            subdirectories: Default::default(),
            files: files.collect(),
        }
    }

    fn size(&self) -> usize {
        let subdir_size = self
            .subdirectories
            .values()
            .fold(0, |acc, dir| acc + dir.size());
        subdir_size + self.files.values().sum::<usize>()
    }

    fn insert(&mut self, path: &str, directory: Directory) {
        if path.is_empty() {
            self.merge(directory);
            return;
        }
        let (head, tail) = path.split_once('/').unwrap_or((path, ""));
        self.subdirectories
            .raw_entry_mut()
            .from_key(head)
            .or_insert(head.to_owned(), Default::default())
            .1
            .insert(tail, directory);
    }

    fn merge(&mut self, mut directory: Directory) {
        self.files.extend(directory.files.drain());
        for (name, subdirectory) in directory.subdirectories.drain() {
            use std::collections::hash_map::Entry;
            match self.subdirectories.entry(name) {
                Entry::Occupied(mut existing) => {
                    existing.get_mut().merge(subdirectory);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(subdirectory);
                }
            };
        }
    }

    fn traverse(&self) -> DirectoryTraverse {
        DirectoryTraverse {
            iterators: vec![Box::new(std::iter::once(("/", self)))],
        }
    }
}

#[derive(PartialEq)]
enum Command {
    ChangeDirectory(String),
    ListDirectory(Directory),
}

fn parse_command(input: &str) -> Command {
    let (command_type, command_result) = input.split_once(char::is_whitespace).unwrap();
    match command_type {
        "cd" => Command::ChangeDirectory(command_result.into()),
        "ls" => Command::ListDirectory(Directory::parse(command_result)),
        x => panic!("Unexpected command {x}"),
    }
}

fn main() {
    let matches = clap::App::new("no-space-left-on-device")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Analyzes directory structures")
        .args_from_usage("<FILENAME>    'The file containing the input'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());

    let input = std::fs::read_to_string(path).unwrap();
    let mut commands = input.split('$').map(str::trim);
    assert!(commands.next().unwrap().is_empty());
    let mut commands = commands.map(parse_command);
    assert!(commands.next().unwrap() == Command::ChangeDirectory("/".to_owned()));

    let mut pwd = "/".to_owned();
    let mut root_directory = Directory::default();
    for command in commands {
        match command {
            Command::ChangeDirectory(path) => match path.as_str() {
                ".." => pwd.truncate(pwd.rfind('/').unwrap()),
                "/" => pwd = "/".to_owned(),
                subdirectory => pwd.push_str(&format!("/{subdirectory}")),
            },
            Command::ListDirectory(directory) => {
                root_directory.insert(&pwd, directory);
            }
        }
    }

    let total_size = root_directory
        .traverse()
        .map(|(_, directory)| directory.size())
        .filter(|&size| size < 100000)
        .sum::<usize>();
    println!("Total size of directories smaller than 100 000: {total_size}");

    let smallest_sufficient_size = root_directory
        .traverse()
        .map(|(_, directory)| directory.size())
        .filter(|&size| size >= root_directory.size() - 40000000)
        .min()
        .unwrap();
    println!("Size of smallest directory of sufficient size: {smallest_sufficient_size}");
}
